use std::fmt::{Debug, Formatter, Error as FmtError};
use std::path::Path;
use std::borrow::Cow;
use std::collections::BTreeMap;

use super::Buf;

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
            NugetTarget::Windows(ref arch) => path("win7", arch.rid()),
            NugetTarget::MacOS(ref arch) => path("osx", arch.rid()),
            NugetTarget::Unix(ref arch) => path("unix", arch.rid()),
            _ => "any".into(),
        }
    }
}

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

#[derive(Debug, PartialEq)]
pub struct NugetPackArgs<'a> {
    pub spec: &'a Buf,
    pub cargo_libs: BTreeMap<NugetTarget, &'a Path>,
}

#[derive(Debug, PartialEq)]
pub struct Nupkg {
    rids: Vec<Cow<'static, str>>,
    buf: Buf,
}

pub fn pack<'a>(args: NugetPackArgs<'a>) -> Result<Nupkg, NugetPackError> {
    // TODO: Build zip: write nuspec, write folder for each lib
    // Windows: /runtimes/win7-x64/native/file.dll

    let pkgs: Vec<_> = args.cargo_libs
        .iter()
        .filter_map(|(target, path)| match target {
            &NugetTarget::Unknown => None,
            target => Some((target.rid(), path)),
        })
        .collect();

    let rids = pkgs.into_iter().map(|(rid, _)| rid).collect();

    Ok(Nupkg {
        rids: rids,
        buf: vec![].into(),
    })
}

quick_error!{
    #[derive(Debug)]
    pub enum NugetPackError {

    }
}
