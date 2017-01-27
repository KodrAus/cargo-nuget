use std::str::{self, Utf8Error};
use std::io::{Read, Error as IoError};
use std::borrow::Cow;
use std::fs::File;
use toml::{Parser, ParserError};

/// Args for parsing a `Cargo.toml` package metadata file.
/// 
/// The source can either be a relative filepath or a byte buffer.
#[derive(Debug, PartialEq)]
pub enum ParseCargoArgs<'a> {
	FromFile { path: Cow<'a, str> },
	FromBuf { buf: Cow<'a, [u8]> }
}

/// The parsed `Cargo.toml` metadata.
#[derive(Debug, PartialEq)]
pub struct CargoConfig {
	pub name: String,
	pub version: String,
	pub authors: Vec<String>,
	pub description: Option<String>
}

macro_rules! toml_val {
    ($toml:ident [ $key:expr ] . $cast:ident ( )) => ({
    	$toml.get($key).and_then(|k| k.$cast()).ok_or(CargoParseError::Missing { key: $key })
    })
}

/// Parse `CargoConfig` from the given source.
pub fn parse_toml<'a, T>(args: T) -> Result<CargoConfig, CargoParseError> 
	where T: Into<ParseCargoArgs<'a>>
{
	let args = args.into();

	// Get a buffer to the toml file
	let buf = match args {
		// Read the file to an owned buffer
		ParseCargoArgs::FromFile { path } => {
			let mut buf = Vec::new();
			let mut f = File::open(path.as_ref())
				.map_err(|e| CargoParseError::Io { src: path.to_string(), err: e })?;

			f.read_to_end(&mut buf)
				.map_err(|e| CargoParseError::Io { src: path.to_string(), err: e })?;

			Cow::Owned(buf)
		},
		// Just use the buffer given
		ParseCargoArgs::FromBuf { buf } => buf
	};

	let utf8 = str::from_utf8(&buf)?;
	let mut parser = Parser::new(utf8);

	// Parse the toml config
	match parser.parse() {
		Some(toml) => {
			let pkg = toml_val!(toml["package"].as_table())?;
			let name = toml_val!(pkg["name"].as_str())?.into();
			let ver = toml_val!(pkg["version"].as_str())?.into();
			let desc = toml_val!(pkg["description"].as_str()).ok().map(|v| v.into());
			let authors = toml_val!(pkg["authors"].as_slice())?
				.iter()
				.filter_map(|a| a.as_str())
				.map(|a| a.into())
				.collect();

			Ok(CargoConfig {
				name: name,
				version: ver,
				authors: authors,
				description: desc
			})
		},
		None => {
			Err(CargoParseError::Toml { errs: parser.errors })
		}
	}
}

quick_error!{
	#[derive(Debug)]
	pub enum CargoParseError {
		/// An io-related error reading from a file.
		Io { src: String, err: IoError } {
			cause(err)
			display("Error reading config from '{}'\nCaused by: {}", src, err)
		}
		/// An error reading the buffer as a UTF8 string.
		Utf8(err: Utf8Error) {
			cause(err)
			display("Error parsing config\nCaused by: {}", err)
			from()
		}
		/// A required value that wasn't in the config.
		/// 
		/// This could be because it isn't present, in the wrong place,
		/// or has the wrong value.
		Missing { key: &'static str } {
			display("The config is missing '{}'", key)
		}
		/// An error parsing the input as TOML.
		Toml { errs: Vec<ParserError> } {
			display("Error parsing config\nCaused by: {:?}", errs)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_toml_from_file() {
		let args = ParseCargoArgs::FromFile { 
			path: Cow::Borrowed("tests/native/Cargo.toml") 
		};

		parse_toml(args).unwrap();
	}

	#[test]
	fn parse_toml_from_buf() {
		let toml = r#"
			[package]
			name = "native"
			version = "0.1.0"
			authors = ["Somebody", "Somebody Else"]
		"#;

		let args = ParseCargoArgs::FromBuf { 
			buf: Cow::Borrowed(toml.as_bytes())
		};

		let toml = parse_toml(args).unwrap();

		let expected = CargoConfig {
			name: "native".into(),
			version: "0.1.0".into(),
			authors: vec!["Somebody".into(), "Somebody Else".into()],
			description: None
		};

		assert_eq!(expected, toml);
	}
}