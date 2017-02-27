use std::fmt::{Debug, Formatter, Error as FmtError};
use std::io::{copy, Cursor, Write, Seek, Error as IoError};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::borrow::Cow;
use std::collections::BTreeMap;
use zip::CompressionMethod;
use zip::write::{ZipWriter, FileOptions};
use zip::result::ZipError;

use super::{xml, Buf};

mod openxml;

/// A target platform for the nuget package.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NugetTarget {
    Unknown,
    Windows(NugetArch),
    Unix(NugetArch),
    MacOS(NugetArch),
}

impl NugetTarget {
    pub fn local() -> Self {
        LOCAL_TARGET
    }

    fn rid(&self) -> Cow<'static, str> {
        fn path(target: &'static str, arch: Option<&'static str>) -> Cow<'static, str> {
            match arch {
                Some(arch) => format!("{}-{}", target, arch).into(),
                None => target.into(),
            }
        }

        match *self {
            NugetTarget::Windows(ref arch) => path("win", arch.rid()),
            NugetTarget::MacOS(ref arch) => path("osx", arch.rid()),
            NugetTarget::Unix(ref arch) => path("unix", arch.rid()),
            _ => "any".into(),
        }
    }
}

/// A target architecture for the nuget package.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NugetArch {
    Unknown,
    x64,
    x86,
}

impl NugetArch {
    pub fn local() -> Self {
        LOCAL_ARCH
    }

    fn rid(&self) -> Option<&'static str> {
        match *self {
            NugetArch::x86 => Some("x86"),
            NugetArch::x64 => Some("x64"),
            NugetArch::Unknown => None,
        }
    }
}

const X86_ARCH: NugetArch = NugetArch::x86;
const X64_ARCH: NugetArch = NugetArch::x64;

#[cfg(target_arch = "x86")]
const LOCAL_ARCH: NugetArch = X86_ARCH;
#[cfg(target_arch = "x86_64")]
const LOCAL_ARCH: NugetArch = X64_ARCH;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
const LOCAL_ARCH: NugetArch = NugetArch::Unknown;

#[cfg(windows)]
const LOCAL_TARGET: NugetTarget = NugetTarget::Windows(LOCAL_ARCH);
#[cfg(macos)]
const LOCAL_TARGET: NugetTarget = NugetTarget::MacOS(LOCAL_ARCH);
#[cfg(unix)]
const LOCAL_TARGET: NugetTarget = NugetTarget::Unix(LOCAL_ARCH);
#[cfg(not(any(windows, macos, unix)))]
const LOCAL_TARGET: NugetTarget = NugetTarget::Unknown;

/// Args for building a `nupkg` with potentially multiple targets.
#[derive(Debug, PartialEq)]
pub struct NugetPackArgs<'a> {
    pub id: Cow<'a, str>,
    pub version: Cow<'a, str>,
    pub spec: &'a Buf,
    pub cargo_libs: BTreeMap<NugetTarget, &'a Path>,
}

/// A formatted `nupkg`.
#[derive(Debug, PartialEq)]
pub struct Nupkg {
    name: String,
    rids: Vec<Cow<'static, str>>,
    buf: Buf,
}

fn options() -> FileOptions {
    FileOptions::default().compression_method(CompressionMethod::Deflated)
}

/// Pack a `nuspec` and native libs into a `nupkg`.
pub fn pack<'a>(args: NugetPackArgs<'a>) -> Result<Nupkg, NugetPackError> {
    let pkgs: Vec<_> = args.cargo_libs
        .iter()
        .filter_map(|(target, path)| match target {
            &NugetTarget::Unknown => None,
            target => Some((target.rid(), path)),
        })
        .collect();

    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));

    let mut nuspec_path = {
        let mut path = PathBuf::new();
        path.set_file_name(args.id.as_ref());
        path.set_extension("nuspec");

        path
    };

    write_rels(&mut writer, &nuspec_path)?;
    write_content_types(&mut writer)?;

    writer.start_file(nuspec_path.to_string_lossy(), options())?;
    writer.write_all(&args.spec)?;

    for &(ref rid, ref lib_path) in &pkgs {
        write_lib(&mut writer, rid, lib_path).map_err(|e| {
                NugetPackError::WriteLib {
                    rid: rid.to_string(),
                    lib_path: lib_path.to_string_lossy().into_owned(),
                    err: e,
                }
            })?;
    }

    let mut buf = writer.finish()?.into_inner();

    let rids = pkgs.into_iter().map(|(rid, _)| rid).collect();
    let name = format!("{}.{}.nupkg", args.id, args.version);

    let mut f = File::create(&name).unwrap();
    f.write_all(&mut buf).unwrap();

    Ok(Nupkg {
        name: name,
        rids: rids,
        buf: buf.into(),
    })
}

/// Write `/runtimes/{rid}/native/{lib}`.
fn write_lib<W>(writer: &mut ZipWriter<W>,
                rid: &str,
                lib_path: &Path)
                -> Result<(), NugetWriteLibError>
    where W: Write + Seek
{
    let file_name = lib_path.file_name()
        .ok_or(NugetWriteLibError::BadPath { path: lib_path.to_string_lossy().into_owned() })?;

    let mut path = PathBuf::new();
    path.push("runtimes");
    path.push(rid);
    path.push("native");
    path.push(file_name);

    writer.start_file(path.to_string_lossy(), options())?;

    let mut lib = File::open(lib_path)?;
    copy(&mut lib, writer)?;

    Ok(())
}

/// Write `/_rels/.rels`.
fn write_rels<W>(writer: &mut ZipWriter<W>, nuspec_path: &Path) -> Result<(), NugetPackError>
    where W: Write + Seek
{
    let (path, xml) = openxml::relationships(&nuspec_path)?;

    writer.start_file(path.to_string_lossy(), options())?;
    writer.write_all(&xml)?;

    Ok(())
}

/// Write `/[Content_Types].xml`.
fn write_content_types<W>(writer: &mut ZipWriter<W>) -> Result<(), NugetPackError>
    where W: Write + Seek
{
    let (path, xml) = openxml::content_types()?;

    writer.start_file(path.to_string_lossy(), options())?;
    writer.write_all(&xml)?;

    Ok(())
}

quick_error!{
    #[derive(Debug)]
    pub enum NugetPackError {
        /// A zip writing error.
        Zip(err: ZipError) {
            display("Error building nupkg\nCaused by: {}", err)
            from()
        }
        /// A general io error.
        Io(err: IoError) {
            display("Error building nupkg\nCaused by: {}", err)
            from()
        }
        /// An xml formatting error.
        Xml(err: xml::Error) {
            display("Error building nupkg\nCaused by: {}", err)
            from()
        }
        /// An error with a specific library.
        WriteLib { rid: String, lib_path: String, err: NugetWriteLibError } {
            display("Error reading lib {} at path {}\nCaused by: {}", rid, lib_path, err)
        }
    }
}

quick_error!{
    #[derive(Debug)]
    pub enum NugetWriteLibError {
        /// A zip writing error.
        Zip(err: ZipError) {
            display("Error reading lib\nCaused by: {}", err)
            from()
        }
        /// A general io error.
        Io(err: IoError) {
            display("Error reading lib\nCaused by: {}", err)
            from()
        }
        /// An error parsing a library path.
        BadPath { path: String } {
            display("Error parsing path '{}'", path)
        }
    }
}
