//! Run a `cargo` command that builds some output.

use std::borrow::Cow;
use std::path::{Path, PathBuf};
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

#[allow(dead_code)]
const WINDOWS_EXTENSION: &'static str = "dll";
#[allow(dead_code)]
const UNIX_EXTENSION: &'static str = "so";
#[allow(dead_code)]
const MACOS_EXTENSION: &'static str = "dylib";

#[cfg(windows)]
const LOCAL_EXTENSION: &'static str = WINDOWS_EXTENSION;
#[cfg(unix)]
const LOCAL_EXTENSION: &'static str = UNIX_EXTENSION;
#[cfg(macos)]
const LOCAL_EXTENSION: &'static str = MACOS_EXTENSION;

#[allow(dead_code)]
const UNIX_PREFIX: &'static str = "lib";

#[cfg(unix)]
const LOCAL_PREFIX: Option<&'static str> = Some(UNIX_PREFIX);
#[cfg(not(unix))]
const LOCAL_PREFIX: Option<&'static str> = None;

impl CargoBuildTarget {
    /// Get the platform specific extension for the build output.
    fn extension(&self) -> &'static str {
        match *self {
            CargoBuildTarget::Local => LOCAL_EXTENSION,
        }
    }

    /// Get the platform specific prefix for the build output.
    fn prefix(&self) -> Option<&'static str> {
        match *self {
            CargoBuildTarget::Local => LOCAL_PREFIX
        }
    }
}

/// Args for running a `cargo` command for the native package.
#[derive(Debug, Clone, PartialEq)]
pub struct CargoBuildArgs<'a> {
    pub work_dir: Cow<'a, Path>,
    pub output_name: Cow<'a, str>,
    pub quiet: bool,
    pub kind: CargoBuildKind,
    pub target: CargoBuildTarget,
    pub profile: CargoBuildProfile,
}

/// The output of the `cargo` command.
#[derive(Debug, Clone, PartialEq)]
pub struct CargoBuildOutput<'a> {
    pub path: Cow<'a, Path>,
    pub target: CargoBuildTarget,
}

pub fn build_lib<'a>(args: CargoBuildArgs<'a>) -> Result<CargoBuildOutput<'a>, CargoBuildError> {
    // Run a specialised command if given, but always run `cargo build`
    let cmds = match args.kind {
        CargoBuildKind::Build => vec![CargoBuildKind::Build],
        kind => vec![kind, CargoBuildKind::Build]
    };

    cargo_commands(&args.work_dir, &cmds, args.profile, args.quiet)?;

    let path = output_path(&args);

    match path.exists() {
        true => {
            Ok(CargoBuildOutput {
                path: path.into(),
                target: args.target,
            })
        }
        false => Err(CargoBuildError::MissingOutput { path: path }),
    }
}

/// Get a path to the expected build output.
fn output_path<'a>(args: &CargoBuildArgs<'a>) -> PathBuf {
    let mut output = PathBuf::new();

    let name = match args.target.prefix() {
        Some(prefix) => {
            let name = format!("{}{}", prefix, args.output_name);
            Cow::Owned(name)
        },
        None => Cow::Borrowed(args.output_name.as_ref())
    };

    output.push(args.work_dir.as_ref());
    output.push("target");
    output.push(args.profile.path());
    output.push(name.as_ref());
    output.set_extension(args.target.extension());

    output
}

fn cargo_commands(work_dir: &Path, kinds: &[CargoBuildKind], profile: CargoBuildProfile, quiet: bool) -> Result<(), CargoBuildError> {
    for kind in kinds {
        cargo_command(work_dir, *kind, profile, quiet)?;
    }

    Ok(())
}

fn cargo_command(work_dir: &Path, kind: CargoBuildKind, profile: CargoBuildProfile, quiet: bool) -> Result<(), CargoBuildError> {
    let mut cargo = Command::new("cargo");

    cargo.current_dir(work_dir);

    if quiet {
        cargo.stdout(Stdio::null());
        cargo.stderr(Stdio::null());
    }
    else {
        cargo.stdout(Stdio::inherit());
        cargo.stderr(Stdio::inherit());
    }

    cargo.arg(match kind {
        CargoBuildKind::Build => "build",
        CargoBuildKind::Test => "test",
    });

    if profile == CargoBuildProfile::Release {
        cargo.arg("--release");
    }

    let output = cargo.output().map_err(|e| CargoBuildError::from(e))?;

    match output.status.success() {
        true => Ok(()),
        false => Err(CargoBuildError::Run)
    }
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
    use std::path::Path;
    use super::*;

    fn local_args() -> CargoBuildArgs<'static> {
        let p: &Path = "tests/native".as_ref();

        CargoBuildArgs {
            work_dir: p.into(),
            output_name: "native_test".into(),
            kind: CargoBuildKind::Build,
            target: CargoBuildTarget::Local,
            profile: CargoBuildProfile::Debug,
            quiet: true,
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
        let args = CargoBuildArgs { output_name: "not_the_output".into(), ..local_args() };

        let result = build_lib(args);

        match result {
            Err(CargoBuildError::MissingOutput { .. }) => (),
            r => panic!("{:?}", r),
        }
    }
}
