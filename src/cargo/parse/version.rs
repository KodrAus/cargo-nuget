//! Update the crate version with a pre tag and build number.

use chrono::UTC;

use semver::{Version, Identifier, SemVerError};

pub fn get_dev_version(ver: &str) -> Result<String, CargoVersionError> {
	let mut ver = Version::parse(ver)?;
	let build = UTC::now().timestamp();

	if build < 0 {
		Err(CargoVersionError::PreEpoch)?;
	}

	let build = build as u64;

	add_pretag(&mut ver, "dev", build);

	Ok(ver.to_string())
}

fn add_pretag(ver: &mut Version, tag: &str, num: u64) {
	if ver.pre.len() == 0 {
		ver.pre.push(Identifier::AlphaNumeric(tag.into()));
	}

	ver.pre.push(Identifier::Numeric(num));

	ver.build = vec![];
}

quick_error!{
	/// An error encountered while updating a semver version.
	#[derive(Debug)]
	pub enum CargoVersionError {
		Parse(err: SemVerError) {
			cause(err)
			display("Error adding dev pretag\nCaused by: {}", err)
            from()
		}
		PreEpoch {
			display("Current timestamp is before the epoch\nYou are either a time traveller or there's an error with your clock")
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use semver::Version;

	#[test]
	fn add_pretag_and_build() {
		let mut ver = Version::parse("0.0.1").unwrap();

		add_pretag(&mut ver, "dev", 2);

		assert_eq!("0.0.1-dev.2", &ver.to_string());
	}

	#[test]
	fn use_existing_pretag() {
		let mut ver = Version::parse("0.0.1-carrots1").unwrap();

		add_pretag(&mut ver, "dev", 2);

		assert_eq!("0.0.1-carrots1.2", &ver.to_string());
	}

	#[test]
	fn use_existing_pretag_ignore_build() {
		let mut ver = Version::parse("0.0.1-carrots+1").unwrap();

		add_pretag(&mut ver, "dev", 2);

		assert_eq!("0.0.1-carrots.2", &ver.to_string());
	}
}