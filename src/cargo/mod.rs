//! Commands for interacting with Cargo and Rust projects.

mod build;
mod parse;

pub use self::build::*;
pub use self::parse::*;

use std::path::PathBuf;
use clap::ArgMatches;

use args::{CARGO_WORK_DIR_ARG, TEST_ARG, RELEASE_ARG};

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

        CargoParseArgs::FromFile { path: path }
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

/// Get the build profile from program input.
impl<'a> From<&'a ArgMatches<'a>> for CargoBuildProfile {
    fn from(args: &'a ArgMatches<'a>) -> Self {
        match args.is_present(RELEASE_ARG) {
            true => CargoBuildProfile::Release,
            _ => CargoBuildProfile::Debug,
        }
    }
}

/// Build args to run a cargo command from program input and toml config.
impl<'a> From<(&'a ArgMatches<'a>, &'a CargoConfig)> for CargoBuildArgs<'a> {
    fn from((args, cargo): (&'a ArgMatches<'a>, &'a CargoConfig)) -> Self {
        let path = match args.value_of(CARGO_WORK_DIR_ARG) {
            Some(path) => path,
            None => "",
        };

        CargoBuildArgs {
            work_dir: path,
            output_name: &cargo.name,
            kind: args.into(),
            target: CargoBuildTarget::Local,
            profile: args.into(),
        }
    }
}
