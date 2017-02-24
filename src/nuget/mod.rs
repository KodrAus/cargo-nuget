//! Commands for interacting with Nuget packages.

mod format;
mod pack;

pub use self::format::*;
pub use self::pack::*;

use std::fmt::{Debug, Formatter, Error as FmtError};
use std::collections::BTreeMap;
use std::borrow::Cow;
use std::ops::Deref;
use cargo::{CargoConfig, CargoBuildTarget, CargoBuildOutput};

#[derive(PartialEq)]
pub struct Buf(Vec<u8>);

impl From<Vec<u8>> for Buf {
	fn from(buf: Vec<u8>) -> Self {
		Buf(buf)
	}
}

impl Deref for Buf {
	type Target = [u8];

	fn deref(&self) -> &[u8] {
		&self.0
	}
}

impl Debug for Buf {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        fmt.debug_struct("Buf").finish()
    }
}

impl<'a> From<&'a CargoConfig> for FormatNuspecArgs<'a> {
    fn from(cargo: &'a CargoConfig) -> Self {
        FormatNuspecArgs {
            id: Cow::Borrowed(&cargo.name),
            version: Cow::Borrowed(&cargo.version),
            authors: Cow::Owned((&cargo.authors).join(", ")),
            description: cargo.description.clone().map(|d| Cow::Owned(d)),
        }
    }
}

impl From<CargoBuildTarget> for NugetTarget {
	fn from(value: CargoBuildTarget) -> NugetTarget {
		match value {
			CargoBuildTarget::Local => NugetTarget::local()
		}
	}
}

impl<'a> From<(&'a Nuspec, &'a CargoBuildOutput)> for NugetPackArgs<'a> {
	fn from((nuspec, build): (&'a Nuspec, &'a CargoBuildOutput)) -> Self {
		let mut libs = BTreeMap::new();

		libs.insert(build.target.into(), build.path.as_ref());

		NugetPackArgs {
			spec: &nuspec.xml,
			cargo_libs: libs
		}
	}
}