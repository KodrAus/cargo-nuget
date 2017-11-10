//! Run a `cargo` command that builds some output.

use std::borrow::Cow;
use std::path::Path;

use super::{CargoBuildOutput, CargoBuildError};
use args::{Action, Profile, Target, CrossTarget};

/// Args for running a `cargo` command for the native package.
#[derive(Debug, Clone, PartialEq)]
pub struct CargoCrossBuildArgs<'a> {
    pub work_dir: Cow<'a, Path>,
    pub quiet: bool,
    pub action: Action,
    pub profile: Profile,
    pub targets: Vec<CargoCrossTarget<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CargoCrossTarget<'a> {
    Build {
        output_name: Cow<'a, str>,
        target: CrossTarget,
    },
    Path {
        target: CrossTarget,
        path: Cow<'a, Path>,
    },
}

pub fn build_cross<'a>(args: CargoCrossBuildArgs<'a>) -> Result<Vec<CargoBuildOutput>, CargoBuildError> {
    args.targets.into_iter().map(|args| {
        match args {
            CargoCrossTarget::Build { .. } => unimplemented!("cross platform builds aren't supported yet"),
            CargoCrossTarget::Path { target, path } => {
                match path.exists() {
                    true => {
                        Ok(CargoBuildOutput {
                            path: path.into_owned(),
                            target: Target::Cross(target),
                        })
                    }
                    false => Err(CargoBuildError::MissingOutput { path: path.into_owned() }),
                }
            }
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;
}
