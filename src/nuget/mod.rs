//! Commands for interacting with Nuget packages.

mod spec;
mod pack;
mod save;

mod util;

pub use self::spec::*;
pub use self::pack::*;
pub use self::save::*;

use std::path::PathBuf;
use std::fmt::{Debug, Error as FmtError, Formatter};
use std::collections::HashMap;
use std::borrow::Cow;
use std::ops::Deref;
use clap::ArgMatches;

use cargo::{CargoBuildOutput, CargoConfig};
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
impl<'a> From<&'a CargoConfig> for NugetSpecArgs<'a> {
    fn from(cargo: &'a CargoConfig) -> Self {
        NugetSpecArgs {
            id: Cow::Borrowed(&cargo.name),
            version: Cow::Borrowed(&cargo.version),
            authors: Cow::Owned((&cargo.authors).join(", ")),
            description: Cow::Borrowed(&cargo.description),
            dependencies: NugetDependencies::default(),
        }
    }
}

/// Build args to pack a nupkg from nuspec and cargo build.
impl<'a, I> From<(&'a Nuspec<'a>, I)> for NugetPackArgs<'a>
where
    I: IntoIterator<Item = &'a CargoBuildOutput>,
{
    fn from((nuspec, builds): (&'a Nuspec, I)) -> Self {
        let mut libs = HashMap::new();

        for build in builds {
            libs.insert(build.target, Cow::Borrowed(build.path.as_ref()));
        }

        NugetPackArgs {
            id: Cow::Borrowed(&nuspec.id),
            version: Cow::Borrowed(&nuspec.version),
            spec: &nuspec.xml,
            cargo_libs: libs,
        }
    }
}

/// Build args to run a cargo command from program input and toml config.
impl<'a> From<(&'a ArgMatches<'a>, &'a Nupkg<'a>)> for NugetSaveArgs<'a> {
    fn from((args, nupkg): (&'a ArgMatches<'a>, &'a Nupkg<'a>)) -> Self {
        let mut path = match args.value_of(NUPKG_DIR_ARG) {
            Some(path) => path.into(),
            None => PathBuf::from("."),
        };

        path.push(nupkg.name.as_ref());

        NugetSaveArgs {
            path: path.into(),
            nupkg: &nupkg.buf,
        }
    }
}
