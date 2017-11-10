//! Run a `cargo` command that builds some output.

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;

use super::{CargoBuildError, CargoBuildOutput};
use args::{Action, CrossTarget, Profile, Target};

/// Args for running a `cargo` command for the native package.
#[derive(Debug, Clone, PartialEq)]
pub struct CargoCrossBuildArgs<'a> {
    pub work_dir: Cow<'a, Path>,
    pub quiet: bool,
    pub targets: HashMap<CrossTarget, CargoCrossTarget<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CargoCrossTarget<'a> {
    Build {
        action: Action,
        profile: Profile,
        output_name: Cow<'a, str>,
    },
    Path(Cow<'a, Path>),
}

pub fn build_cross<'a>(
    args: CargoCrossBuildArgs<'a>,
) -> Result<Vec<CargoBuildOutput>, CargoBuildError> {
    args.targets
        .into_iter()
        .map(|(target, args)| match args {
            CargoCrossTarget::Build { .. } => Err(CargoBuildError::UnsupportedCrossBuild),
            CargoCrossTarget::Path(path) => match path.exists() {
                true => Ok(CargoBuildOutput {
                    path: path.into_owned(),
                    target: Target::Cross(target),
                }),
                false => Err(CargoBuildError::MissingOutput {
                    path: path.into_owned(),
                }),
            },
        })
        .collect::<Result<Vec<_>, CargoBuildError>>()
        .and_then(|builds| match builds.len() {
            0 => Err(CargoBuildError::NoValidTargets),
            _ => Ok(builds),
        })
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use args::*;
    use super::*;

    fn empty_args() -> CargoCrossBuildArgs<'static> {
        let p: &Path = "tests/native".as_ref();

        CargoCrossBuildArgs {
            work_dir: p.into(),
            targets: HashMap::new(),
            quiet: true,
        }
    }

    #[test]
    fn cargo_cross_paths() {
        let mut targets = HashMap::new();

        targets.insert(
            CrossTarget::Windows(Arch::x64),
            CargoCrossTarget::Path(Cow::Owned("Cargo.toml".into())),
        );
        targets.insert(
            CrossTarget::Linux(Arch::x64),
            CargoCrossTarget::Path(Cow::Owned("Cargo.toml".into())),
        );
        targets.insert(
            CrossTarget::MacOS(Arch::x64),
            CargoCrossTarget::Path(Cow::Owned("Cargo.toml".into())),
        );

        let args = CargoCrossBuildArgs {
            targets: targets,
            ..empty_args()
        };

        build_cross(args).unwrap();
    }

    #[test]
    fn cargo_cross_build() {
        let mut targets = HashMap::new();

        targets.insert(
            CrossTarget::Windows(Arch::x64),
            CargoCrossTarget::Path(Cow::Owned("Cargo.toml".into())),
        );
        targets.insert(
            CrossTarget::MacOS(Arch::x64),
            CargoCrossTarget::Build {
                action: Action::Build,
                profile: Profile::Debug,
                output_name: "some_output".into(),
            },
        );

        let args = CargoCrossBuildArgs {
            targets: targets,
            ..empty_args()
        };

        let result = build_cross(args);

        match result {
            Err(CargoBuildError::UnsupportedCrossBuild) => (),
            r => panic!("{:?}", r),
        }
    }

    #[test]
    fn cargo_cross_empty_targets() {
        let args = empty_args();

        let result = build_cross(args);

        match result {
            Err(CargoBuildError::NoValidTargets) => (),
            r => panic!("{:?}", r),
        }
    }

    #[test]
    fn cargo_cross_missing_output() {
        let mut targets = HashMap::new();

        targets.insert(
            CrossTarget::Windows(Arch::x64),
            CargoCrossTarget::Path(Cow::Owned("not the output".into())),
        );

        let args = CargoCrossBuildArgs {
            targets: targets,
            ..empty_args()
        };

        let result = build_cross(args);

        match result {
            Err(CargoBuildError::MissingOutput { .. }) => (),
            r => panic!("{:?}", r),
        }
    }
}
