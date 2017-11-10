use std::path::PathBuf;
use std::borrow::Cow;
use std::io::Error as IoError;
use clap::ArgMatches;
use args::{CARGO_BUILD_QUIET_ARG, CARGO_WORK_DIR_ARG, TEST_ARG, RELEASE_ARG, TARGETS_ARG, target_path_arg, Action, Profile, Target, CrossTarget};
use super::CargoConfig;

mod local;
mod cross;

pub use self::local::*;
pub use self::cross::*;

impl Profile {
    /// Get the path within the `target` folder for the output.
    fn path(&self) -> &'static str {
        match *self {
            Profile::Debug => "debug",
            Profile::Release => "release",
        }
    }
}

impl CrossTarget {
    /// Get the platform specific extension for the build output.
    fn extension(&self) -> &'static str {
        match *self {
            CrossTarget::Windows(_) => "dll",
            CrossTarget::Linux(_) => "dll",
            CrossTarget::MacOS(_) => "dylib",
        }
    }

    /// Get the platform specific prefix for the build output.
    fn prefix(&self) -> Option<&'static str> {
        match *self {
            CrossTarget::Windows(_) => None,
            CrossTarget::Linux(_) => Some("lib"),
            CrossTarget::MacOS(_) => Some("lib"),
        }
    }
}

fn parse_targets<'a>(args: &'a ArgMatches<'a>) -> Vec<CrossTarget> {
    args.values_of(TARGETS_ARG)
        .map(Iterator::collect)
        .unwrap_or_else(Vec::new)
        .into_iter()
        .filter_map(|target| match CrossTarget::from_rid(target) {
            Some(target) => Some(target),
            None => {
                warn!("'{}' could not be parsed to an rid", target);
                None
            }
        })
        .collect()
}

fn target_path<'a>(args: &'a ArgMatches<'a>, target: CrossTarget) -> Option<PathBuf> {
    let arg = target_path_arg(target);

    args.value_of(arg).map(Into::into)
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

        let quiet = args.is_present(CARGO_BUILD_QUIET_ARG);

        CargoLocalBuildArgs {
            work_dir: path.into(),
            output_name: Cow::Borrowed(&cargo.name),
            action: action,
            profile: profile,
            quiet: quiet,
        }
    }
}

/// Build args to run a cargo command from program input and toml config.
impl<'a> From<(&'a ArgMatches<'a>, &'a CargoConfig)> for CargoCrossBuildArgs<'a> {
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

        let quiet = args.is_present(CARGO_BUILD_QUIET_ARG);

        let targets = parse_targets(args).into_iter().map(|target| {
            match target_path(args, target) {
                Some(path) => CargoCrossTarget::Path {
                    target: target,
                    path: path.into(),
                },
                None => CargoCrossTarget::Build {
                    target: target,
                    output_name: Cow::Borrowed(&cargo.name)
                }
            }
        }).collect();

        CargoCrossBuildArgs {
            work_dir: path.into(),
            action: action,
            profile: profile,
            quiet: quiet,
            targets: targets,
        }
    }
}

/// The output of the `cargo` command.
#[derive(Debug, Clone, PartialEq)]
pub struct CargoBuildOutput {
    pub path: PathBuf,
    pub target: Target,
}

quick_error!{
    /// An error encountered while parsing Cargo configuration.
    #[derive(Debug)]
    pub enum CargoBuildError {
        /// An io-related error reading from a file.
        Io (err: IoError) {
            cause(err)
            display("Error running cargo build\nCaused by: {}", err)
            from()
        }
        /// An error running a cargo command.
        Run {
            display("Error running cargo build\nBuild output (if any) should be written to stderr")
        }
        /// An error getting a concrete target to build for.
        NoValidTargets {
            display("No valid platform targets were supplied\nThis probably means you're running on an unsupported platform\nOr didn't supply any targets to build")
        }
        /// An error attempting to find the build output.
        MissingOutput { path: PathBuf } {
            display("Build output was expected to be at {:?} but wasn't found", path)
        }
    }
}
