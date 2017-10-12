//! Run a `cargo` command that builds some output.

use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::Error as IoError;

use args::{Action, Profile, Target, CrossTarget};

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

/// Args for running a `cargo` command for the native package.
#[derive(Debug, Clone, PartialEq)]
pub struct CargoLocalBuildArgs<'a> {
    pub work_dir: Cow<'a, Path>,
    pub output_name: Cow<'a, str>,
    pub quiet: bool,
    pub kind: Action,
    pub profile: Profile,
}

/// The output of the `cargo` command.
#[derive(Debug, Clone, PartialEq)]
pub struct CargoBuildOutput {
    pub path: PathBuf,
    pub target: Target,
}

pub fn build_local<'a>(args: CargoLocalBuildArgs<'a>) -> Result<CargoBuildOutput, CargoLocalBuildError> {
    let target = Target::Local;

    // Run a specialised command if given, but always run `cargo build`
    let cmds = match args.kind {
        Action::Build => vec![Action::Build],
        kind => vec![kind, Action::Build],
    };

    cargo_commands(&args.work_dir, &cmds, args.profile, args.quiet)?;

    let path = output_path(&args, target.cross().ok_or(CargoLocalBuildError::UnknownTarget)?);

    match path.exists() {
        true => {
            Ok(CargoBuildOutput {
                path: path,
                target: target,
            })
        }
        false => Err(CargoLocalBuildError::MissingOutput { path: path }),
    }
}

/// Get a path to the expected build output.
fn output_path<'a>(args: &CargoLocalBuildArgs<'a>, target: CrossTarget) -> PathBuf {
    let mut output = PathBuf::new();

    let name = match target.prefix() {
        Some(prefix) => {
            let name = format!("{}{}", prefix, args.output_name);
            Cow::Owned(name)
        }
        None => Cow::Borrowed(args.output_name.as_ref()),
    };

    output.push(args.work_dir.as_ref());
    output.push("target");
    output.push(args.profile.path());
    output.push(name.as_ref());
    output.set_extension(target.extension());

    output
}

fn cargo_commands(work_dir: &Path,
                  kinds: &[Action],
                  profile: Profile,
                  quiet: bool)
                  -> Result<(), CargoLocalBuildError> {
    for kind in kinds {
        cargo_command(work_dir, *kind, profile, quiet)?;
    }

    Ok(())
}

fn cargo_command(work_dir: &Path,
                 kind: Action,
                 profile: Profile,
                 quiet: bool)
                 -> Result<(), CargoLocalBuildError> {
    let mut cargo = Command::new("cargo");

    cargo.current_dir(work_dir);

    if quiet {
        cargo.stdout(Stdio::null());
        cargo.stderr(Stdio::null());
    } else {
        cargo.stdout(Stdio::inherit());
        cargo.stderr(Stdio::inherit());
    }

    cargo.arg(match kind {
        Action::Build => "build",
        Action::Test => "test",
    });

    if profile == Profile::Release {
        cargo.arg("--release");
    }

    let output = cargo.output()?;

    match output.status.success() {
        true => Ok(()),
        false => Err(CargoLocalBuildError::Run),
    }
}

quick_error!{
    /// An error encountered while parsing Cargo configuration.
    #[derive(Debug)]
    pub enum CargoLocalBuildError {
        /// An io-related error reading from a file.
        Io (err: IoError) {
            cause(err)
            display("Error running cargo build\nCaused by: {}", err)
            from()
        }
        Run {
            display("Error running cargo build\nBuild output (if any) should be written to stderr")
        }
        UnknownTarget {
            display("Unknown build target\nThis probably means you're running on an unsupported platform")
        }
        MissingOutput { path: PathBuf } {
            display("Build output was expected to be at {:?} but wasn't found", path)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    fn local_args() -> CargoLocalBuildArgs<'static> {
        let p: &Path = "tests/native".as_ref();

        CargoLocalBuildArgs {
            work_dir: p.into(),
            output_name: "native_test".into(),
            kind: Action::Build,
            target: Target::Local,
            profile: Profile::Debug,
            quiet: true,
        }
    }

    #[test]
    fn local_target_extension() {
        assert_eq!(LOCAL_EXTENSION, Target::Local.extension());
        assert_eq!(LOCAL_EXTENSION, Target::Cross(CrossTarget::local()).extension());
    }

    #[test]
    fn local_target_prefix() {
        assert_eq!(LOCAL_PREFIX, Target::Local.prefix());
        assert_eq!(LOCAL_PREFIX, Target::Cross(CrossTarget::local()).prefix());
    }

    #[test]
    fn cargo_build_debug() {
        let args = local_args();

        build_lib(args).unwrap();
    }

    #[test]
    fn cargo_build_release() {
        let args = CargoLocalBuildArgs { profile: Profile::Release, ..local_args() };

        build_lib(args).unwrap();
    }

    #[test]
    fn cargo_test_debug() {
        let args = CargoLocalBuildArgs { kind: Action::Test, ..local_args() };

        build_lib(args).unwrap();
    }

    #[test]
    fn cargo_test_release() {
        let args = CargoLocalBuildArgs {
            kind: Action::Test,
            profile: Profile::Release,
            ..local_args()
        };

        build_lib(args).unwrap();
    }

    #[test]
    fn cargo_build_missing_output() {
        let args = CargoLocalBuildArgs { output_name: "not_the_output".into(), ..local_args() };

        let result = build_lib(args);

        match result {
            Err(CargoLocalBuildError::MissingOutput { .. }) => (),
            r => panic!("{:?}", r),
        }
    }
}
