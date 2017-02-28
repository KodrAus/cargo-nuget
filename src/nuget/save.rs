use std::path::PathBuf;
use std::io::{Write, Error as IoError};
use std::fs::OpenOptions;

use super::Buf;

/// Args for saving a `nupkg` to a file.
#[derive(Debug, PartialEq)]
pub struct NugetSaveArgs<'a> {
    pub path: PathBuf,
    pub nupkg: &'a Buf,
}

/// A saved `nupkg`.
#[derive(Debug, PartialEq)]
pub struct NupkgPath {
    pub path: PathBuf,
}

/// Format the input as a `nuspec` xml buffer.
pub fn save_nupkg<'a>(args: NugetSaveArgs<'a>) -> Result<NupkgPath, NugetSaveError> {
    let mut f = OpenOptions::new().write(true)
        .truncate(true)
        .create(true)
        .open(&args.path)?;

    f.write_all(&args.nupkg)?;

    Ok(NupkgPath { path: args.path })
}

quick_error!{
	#[derive(Debug)]
	pub enum NugetSaveError {
		/// An io-related error writing to a file.
        Io (err: IoError) {
            cause(err)
            display("Error saving nupkg\nCaused by: {}", err)
            from()
        }
	}
}
