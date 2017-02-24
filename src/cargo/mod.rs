//! Commands for interacting with Cargo and Rust projects.

mod build;
mod parse;

pub use self::build::*;
pub use self::parse::*;

use std::path::PathBuf;
use clap::ArgMatches;

use {CARGO_WORK_DIR_ARG, TEST_ARG, RELEASE_ARG};

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
			},
			None => "Cargo.toml".into()
		};

		CargoParseArgs::FromFile { path: path }
	}
}

impl<'a> From<&'a ArgMatches<'a>> for CargoBuildKind {
	fn from(args: &'a ArgMatches<'a>) -> Self {
		match args.is_present(TEST_ARG) {
			true => CargoBuildKind::Test,
			_ => CargoBuildKind::Build
		}
	}
}

impl<'a> From<&'a ArgMatches<'a>> for CargoBuildProfile {
	fn from(args: &'a ArgMatches<'a>) -> Self {
		match args.is_present(RELEASE_ARG) {
			true => CargoBuildProfile::Release,
			_ => CargoBuildProfile::Debug
		}
	}
}

impl<'a> From<(&'a ArgMatches<'a>, &'a CargoConfig)> for CargoBuildArgs<'a> {
	fn from((args, cargo): (&'a ArgMatches<'a>, &'a CargoConfig)) -> Self {
		let path = match args.value_of(CARGO_WORK_DIR_ARG) {
			Some(path) => path,
			None => ""
		};

		build::CargoBuildArgs {
			work_dir: path,
			output_name: &cargo.name,
			kind: args.into(),
			target: CargoBuildTarget::Local,
			profile: args.into()
		}
	}
}
