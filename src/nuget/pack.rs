use std::fmt::{Debug, Formatter, Error as FmtError};
use std::path::Path;
use std::borrow::Cow;
use std::collections::BTreeMap;

use super::Buf;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NugetTarget {
	Windows,
	Debian,
	MacOS
}

impl NugetTarget {
	pub fn local() -> Self {
		LOCAL_TARGET
	}

	// TODO: Support more unix
	pub fn path(&self) -> &'static str {
		match *self {
			NugetTarget::Windows => "win7-x64",
			NugetTarget::Debian => "debian-x64",
			NugetTarget::MacOS => "osx",
		}
	}
}

const WINDOWS_TARGET: NugetTarget = NugetTarget::Windows;
const DEBIAN_TARGET: NugetTarget = NugetTarget::Debian;
const MACOS_TARGET: NugetTarget = NugetTarget::MacOS;

#[cfg(windows)]
const LOCAL_TARGET: NugetTarget = WINDOWS_TARGET;
#[cfg(mac_os)]
const LOCAL_TARGET: NugetTarget = MACOS_TARGET;

// TODO: Proper cfgs for Debian / Fedora
#[cfg(target_os = "debian")]
const LOCAL_TARGET: NugetTarget = DEBIAN_TARGET;

#[derive(Debug, PartialEq)]
pub struct NugetPackArgs<'a> {
    pub spec: &'a Buf,
    pub cargo_libs: BTreeMap<NugetTarget, &'a Path>,
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