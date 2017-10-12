//! Commands for interacting with Cargo and Rust projects.

mod build_local;
mod parse;
mod version;

pub use self::build_local::*;
pub use self::parse::*;
pub use self::version::*;

use std::borrow::Cow;
use std::path::PathBuf;
use clap::ArgMatches;

use args::{CARGO_BUILD_QUIET, CARGO_WORK_DIR_ARG, TEST_ARG, RELEASE_ARG, Action, Profile};

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

        CargoParseArgs { buf: CargoBufKind::FromFile { path: path } }
    }
}

/// Build args to add a dev tag from toml config.
impl<'a> From<&'a CargoConfig> for CargoLocalVersionArgs<'a> {
    fn from(cargo: &'a CargoConfig) -> Self {
        CargoLocalVersionArgs { version: &cargo.version }
    }
}

/// Build args to run a cargo command from program input and toml config.
impl<'a> From<(&'a ArgMatches<'a>, &'a CargoConfig)> for CargoLocalBuildArgs<'a> {
    fn from((args, cargo): (&'a ArgMatches<'a>, &'a CargoConfig)) -> Self {
        let action = match args.is_present(TEST_ARG) {
            true => Action::Test,
            _ => Action::Build,
        };

        let profile = match args.is_present(RELEASE_ARG) {
            true => Profile::Release,
            _ => Profile::Debug,
        };

        let path = match args.value_of(CARGO_WORK_DIR_ARG) {
            Some(path) => path.into(),
            None => PathBuf::from("."),
        };

        let quiet = args.is_present(CARGO_BUILD_QUIET);

        CargoLocalBuildArgs {
            work_dir: path.into(),
            output_name: Cow::Borrowed(&cargo.name),
            kind: action,
            profile: profile,
            quiet: quiet,
        }
    }
}
