//! Run a `cargo` command that builds some output.

use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::{CargoBuildError, CargoBuildOutput};
use args::{Action, CrossTarget, Profile, Target};

/// Args for running a `cargo` command for the native package.
#[derive(Debug, Clone, PartialEq)]
pub struct CargoLocalBuildArgs<'a> {
    pub work_dir: Cow<'a, Path>,
    pub output_name: Cow<'a, str>,
    pub quiet: bool,
    pub action: Action,
    pub profile: Profile,
}

pub fn build_local<'a>(args: CargoLocalBuildArgs<'a>) -> Result<CargoBuildOutput, CargoBuildError> {
    let target = Target::Local;

    // Run a specialised command if given, but always run `cargo build`
    let cmds = match args.action {
        Action::Build => vec![Action::Build],
        action => vec![action, Action::Build],
    };

    cargo_commands(&args.work_dir, &cmds, args.profile, args.quiet)?;

    let path = output_path(
        &args,
        target.cross().ok_or(CargoBuildError::NoValidTargets)?,
    );

    match path.exists() {
        true => Ok(CargoBuildOutput {
            path: path,
            target: target,
        }),
        false => Err(CargoBuildError::MissingOutput { path: path }),
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

fn cargo_commands(
    work_dir: &Path,
    kinds: &[Action],
    profile: Profile,
    quiet: bool,
) -> Result<(), CargoBuildError> {
    for kind in kinds {
        cargo_command(work_dir, *kind, profile, quiet)?;
    }

    Ok(())
}

fn cargo_command(
    work_dir: &Path,
    kind: Action,
    profile: Profile,
    quiet: bool,
) -> Result<(), CargoBuildError> {
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
        false => Err(CargoBuildError::Run),
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
            action: Action::Build,
            profile: Profile::Debug,
            quiet: true,
        }
    }

    #[test]
    fn cargo_build_debug() {
        let args = local_args();

        build_local(args).unwrap();
    }

    #[test]
    fn cargo_build_release() {
        let args = CargoLocalBuildArgs {
            profile: Profile::Release,
            ..local_args()
        };

        build_local(args).unwrap();
    }

    #[test]
    fn cargo_test_debug() {
        let args = CargoLocalBuildArgs {
            action: Action::Test,
            ..local_args()
        };

        build_local(args).unwrap();
    }

    #[test]
    fn cargo_test_release() {
        let args = CargoLocalBuildArgs {
            action: Action::Test,
            profile: Profile::Release,
            ..local_args()
        };

        build_local(args).unwrap();
    }

    #[test]
    fn cargo_build_missing_output() {
        let args = CargoLocalBuildArgs {
            output_name: "not_the_output".into(),
            ..local_args()
        };

        let result = build_local(args);

        match result {
            Err(CargoBuildError::MissingOutput { .. }) => (),
            r => panic!("{:?}", r),
        }
    }
}
