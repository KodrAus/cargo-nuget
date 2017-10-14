//! Commands for interacting with Cargo and Rust projects.

mod build;
mod parse;
mod version;

pub use self::build::*;
pub use self::parse::*;
pub use self::version::*;

use std::path::PathBuf;
use clap::ArgMatches;

use args::CARGO_WORK_DIR_ARG;

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
