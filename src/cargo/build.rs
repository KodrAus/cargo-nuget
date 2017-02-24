//! Run a `cargo` command that builds some output.

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::Error as IoError;

/// The kind of cargo command to run.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CargoBuildKind {
    Build,
    Test,
}

/// The cargo build profile to use for the command.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CargoBuildProfile {
    Debug,
    Release,
}

impl CargoBuildProfile {
    /// Get the path within the `target` folder for the output.
    fn path(&self) -> &'static str {
        match *self {
            CargoBuildProfile::Debug => "debug",
            CargoBuildProfile::Release => "release",
        }
    }
}

/// The platform to target for the output.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CargoBuildTarget {
    Local,
}

const WINDOWS_EXTENSION: &'static str = "dll";
const UNIX_EXTENSION: &'static str = "so";
const MACOS_EXTENSION: &'static str = "dylib";

#[cfg(windows)]
const LOCAL_EXTENSION: &'static str = WINDOWS_EXTENSION;
#[cfg(unix)]
const LOCAL_EXTENSION: &'static str = UNIX_EXTENSION;
#[cfg(macos)]
const LOCAL_EXTENSION: &'static str = MACOS_EXTENSION;

impl CargoBuildTarget {
    /// Get the platform specific extension for the build output.
    fn extension(&self) -> &'static str {
        match *self {
            CargoBuildTarget::Local => LOCAL_EXTENSION,
        }
    }
}

/// Args for running a `cargo` command for the native package.
#[derive(Debug, Clone, PartialEq)]
pub struct CargoBuildArgs<'a> {
    pub work_dir: &'a str,
    pub output_name: &'a str,
    pub kind: CargoBuildKind,
    pub target: CargoBuildTarget,
    pub profile: CargoBuildProfile,
}

/// The output of the `cargo` command.
#[derive(Debug, Clone, PartialEq)]
pub struct CargoBuildOutput {
    pub path: PathBuf,
    pub target: CargoBuildTarget,
}

pub fn build_lib<'a>(args: CargoBuildArgs<'a>) -> Result<CargoBuildOutput, CargoBuildError> {
    let output = cargo_command(&args).output()
        .map_err(|e| CargoBuildError::from(e))?;

    if !output.status.success() {
        Err(CargoBuildError::Run)?;
    }

    let path = output_path(&args);

    match path.exists() {
        true => {
            Ok(CargoBuildOutput {
                path: path,
                target: args.target,
            })
        }
        false => Err(CargoBuildError::MissingOutput { path: path }),
    }
}

/// Get a path to the expected build output.
fn output_path<'a>(args: &CargoBuildArgs<'a>) -> PathBuf {
    let mut output = PathBuf::new();

    output.push(args.work_dir);
    output.push("target");
    output.push(args.profile.path());
    output.push(args.output_name);
    output.set_extension(args.target.extension());

    output
}

fn cargo_command<'a>(args: &CargoBuildArgs<'a>) -> Command {
    let mut cargo = Command::new("cargo");

    cargo.current_dir(&args.work_dir);
    cargo.stdout(Stdio::inherit());
    cargo.stderr(Stdio::inherit());

    cargo.arg(match args.kind {
        CargoBuildKind::Build => "build",
        CargoBuildKind::Test => "test",
    });

    if args.profile == CargoBuildProfile::Release {
        cargo.arg("--release");
    }

    cargo
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
        Run {
            display("Error running cargo build\nBuild output (if any) should be written to stderr")
        }
        MissingOutput { path: PathBuf } {
            display("Build output was expected to be at {:?} but wasn't found", path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local_args() -> CargoBuildArgs<'static> {
        CargoBuildArgs {
            work_dir: "tests/native",
            output_name: "native_test",
            kind: CargoBuildKind::Build,
            target: CargoBuildTarget::Local,
            profile: CargoBuildProfile::Debug,
        }
    }

    #[test]
    fn cargo_build_debug() {
        let args = local_args();

        build_lib(args).unwrap();
    }

    #[test]
    fn cargo_build_release() {
        let args = CargoBuildArgs { profile: CargoBuildProfile::Release, ..local_args() };

        build_lib(args).unwrap();
    }

    #[test]
    fn cargo_test_debug() {
        let args = CargoBuildArgs { kind: CargoBuildKind::Test, ..local_args() };

        build_lib(args).unwrap();
    }

    #[test]
    fn cargo_test_release() {
        let args = CargoBuildArgs {
            kind: CargoBuildKind::Test,
            profile: CargoBuildProfile::Release,
            ..local_args()
        };

        build_lib(args).unwrap();
    }

    #[test]
    fn cargo_build_missing_output() {
        let args = CargoBuildArgs { output_name: "not_the_output", ..local_args() };

        let result = build_lib(args);

        match result {
            Err(CargoBuildError::MissingOutput { .. }) => (),
            r => panic!("{:?}", r),
        }
    }
}
