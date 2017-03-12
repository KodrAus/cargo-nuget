//! Commands for interacting with Cargo and Rust projects.

mod build;
mod parse;

pub use self::build::*;
pub use self::parse::*;

use std::borrow::Cow;
use std::path::PathBuf;
use clap::ArgMatches;

use args::{CARGO_BUILD_QUIET, CARGO_WORK_DIR_ARG, TEST_ARG, RELEASE_ARG};

/// Build args to parse toml from program input.
impl<'a> From<&'a ArgMatches<'a>> for CargoParseArgs<'a> {
    fn from(args: &'a ArgMatches<'a>) -> Self {
        let path = match args.value_of(CARGO_WORK_DIR_ARG) {
            Some(work_dir) => {
                let mut path = PathBuf::new();
                path.push(work_dir);
                path.push("Cargo");
                path.set_extension("toml");

                path.to_string_lossy()
                    .into_owned()
                    .into()
            }
            None => "Cargo.toml".into(),
        };

        let is_release = args.is_present(RELEASE_ARG);

        CargoParseArgs {
            dev: !is_release,
            buf: CargoBufKind::FromFile { path: path }
        }
    }
}

/// Get the build kind from program input.
impl<'a> From<&'a ArgMatches<'a>> for CargoBuildKind {
    fn from(args: &'a ArgMatches<'a>) -> Self {
        match args.is_present(TEST_ARG) {
            true => CargoBuildKind::Test,
            _ => CargoBuildKind::Build,
        }
    }
}

/// Get the crate version from metadata.
impl<'a> From<&'a ArgMatches<'a>> for CargoBuildProfile {
    fn from(args: &'a ArgMatches<'a>) -> Self {
        match args.is_present(RELEASE_ARG) {
            true => CargoBuildProfile::Release,
            _ => CargoBuildProfile::Debug,
        }
    }
}

/// Build args to run a cargo command from program input and toml config.
impl<'a> From<(&'a ArgMatches<'a>, &'a CargoConfig<'a>)> for CargoBuildArgs<'a> {
    fn from((args, cargo): (&'a ArgMatches<'a>, &'a CargoConfig<'a>)) -> Self {
        let path = match args.value_of(CARGO_WORK_DIR_ARG) {
            Some(path) => path.into(),
            None => PathBuf::from("."),
        };

        let quiet = args.is_present(CARGO_BUILD_QUIET);

        CargoBuildArgs {
            work_dir: path.into(),
            output_name: Cow::Borrowed(&cargo.name),
            kind: args.into(),
            target: CargoBuildTarget::Local,
            profile: args.into(),
            quiet: quiet,
        }
    }
}
