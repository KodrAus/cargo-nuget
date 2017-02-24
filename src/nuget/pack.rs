use std::fmt::{Debug, Formatter, Error as FmtError};
use std::path::Path;
use std::borrow::Cow;
use std::collections::BTreeMap;

use super::Buf;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NugetTarget {
    Unknown,
    Windows,
    Unix,
    MacOS,
}

impl NugetTarget {
    pub fn local() -> Self {
        LOCAL_TARGET
    }

    fn runtime(&self) -> &'static str {
        match *self {
            NugetTarget::Windows => "win7",
            NugetTarget::MacOS => "osx",
            NugetTarget::Unix => "unix",
            NugetTarget::Unknown => "unknown",
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

    fn arch(&self) -> &'static str {
        match *self {
            NugetArch::x86 => "x86",
            NugetArch::x64 => "x64",
            NugetArch::Unknown => "unknown",
        }
    }
}

fn runtime_path(target: NugetTarget, arch: NugetArch) -> String {
    format!("{}-{}", target.runtime(), arch.arch())
}

const X86_ARCH: NugetArch = NugetArch::x86;
const X64_ARCH: NugetArch = NugetArch::x64;

#[cfg(target_arch = "x86")]
const LOCAL_ARCH: NugetArch = X86_ARCH;
#[cfg(target_arch = "x86_64")]
const LOCAL_ARCH: NugetArch = X64_ARCH;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
const LOCAL_ARCH: NugetArch = NugetArch::Unknown;

const WINDOWS_TARGET: NugetTarget = NugetTarget::Windows;
const UNIX_TARGET: NugetTarget = NugetTarget::Unix;
const MACOS_TARGET: NugetTarget = NugetTarget::MacOS;

#[cfg(windows)]
const LOCAL_TARGET: NugetTarget = WINDOWS_TARGET;
#[cfg(macos)]
const LOCAL_TARGET: NugetTarget = MACOS_TARGET;
#[cfg(unix)]
const LOCAL_TARGET: NugetTarget = UNIX_TARGET;

#[derive(Debug, PartialEq)]
pub struct NugetPackArgs<'a> {
    pub spec: &'a Buf,
    pub cargo_libs: BTreeMap<(NugetTarget, NugetArch), &'a Path>,
}

#[derive(Debug, PartialEq)]
pub struct Nupkg {
    buf: Buf
}

pub fn pack<'a>(args: NugetPackArgs<'a>) -> Result<Nupkg, NugetPackError> {
    // TODO: Build zip: write nuspec, write folder for each lib
    // Windows: /runtimes/win7-x64/native/file.dll
    unimplemented!();
}

quick_error!{
    #[derive(Debug)]
    pub enum NugetPackError {

    }
}