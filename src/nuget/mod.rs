//! Commands for interacting with Nuget packages.

mod format;
mod pack;
mod save;

mod xml;

pub use self::format::*;
pub use self::pack::*;
pub use self::save::*;

use std::path::PathBuf;
use std::fmt::{Debug, Formatter, Error as FmtError};
use std::collections::BTreeMap;
use std::borrow::Cow;
use std::ops::Deref;
use clap::ArgMatches;

use cargo::{CargoConfig, CargoBuildTarget, CargoBuildOutput};
use args::NUPKG_DIR_ARG;

/// A wrapper around an owned byte buffer.
///
/// This type basically only exists so buffer contents aren't printed
/// in `Debug` output.
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

/// Build args to format a nuspec from cargo toml.
impl<'a> From<&'a CargoConfig> for FormatNuspecArgs<'a> {
    fn from(cargo: &'a CargoConfig) -> Self {
        FormatNuspecArgs {
            id: Cow::Borrowed(&cargo.name),
            version: Cow::Borrowed(&cargo.version),
            authors: Cow::Owned((&cargo.authors).join(", ")),
            description: Cow::Borrowed(&cargo.description),
        }
    }
}

/// Get a target, arch tuple from a cargo build target.
impl From<CargoBuildTarget> for NugetTarget {
    fn from(value: CargoBuildTarget) -> NugetTarget {
        match value {
            CargoBuildTarget::Local => NugetTarget::local(),
        }
    }
}

/// Build args to pack a nupkg from nuspec and cargo build.
impl<'a> From<(&'a Nuspec<'a>, &'a CargoBuildOutput)> for NugetPackArgs<'a> {
    fn from((nuspec, build): (&'a Nuspec, &'a CargoBuildOutput)) -> Self {
        let mut libs = BTreeMap::new();

        libs.insert(build.target.into(), build.path.as_ref());

        NugetPackArgs {
            id: Cow::Borrowed(&nuspec.id),
            version: Cow::Borrowed(&nuspec.version),
            spec: &nuspec.xml,
            cargo_libs: libs,
        }
    }
}

/// Build args to run a cargo command from program input and toml config.
impl<'a> From<(&'a ArgMatches<'a>, &'a Nupkg)> for NugetSaveArgs<'a> {
    fn from((args, nupkg): (&'a ArgMatches<'a>, &'a Nupkg)) -> Self {
        let mut path = match args.value_of(NUPKG_DIR_ARG) {
            Some(path) => path.into(),
            None => PathBuf::new(),
        };

        path.push(&nupkg.name);

        NugetSaveArgs {
            path: path,
            nupkg: &nupkg.buf,
        }
    }
}
