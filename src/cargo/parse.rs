//! Parse metadata out of a `Cargo.toml`. 

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
    FromFile { path: &'a str },
    FromBuf { buf: Cow<'a, [u8]> },
}

/// The type of crate.
#[derive(Debug, PartialEq)]
enum CargoCrateType {
    Unknown,
    RLib,
    DyLib,
}

impl<'a> From<&'a str> for CargoCrateType {
    fn from(value: &'a str) -> CargoCrateType {
        match value {
            "rlib" => CargoCrateType::RLib,
            "dylib" => CargoCrateType::DyLib,
            _ => CargoCrateType::Unknown,
        }
    }
}

/// The parsed `Cargo.toml` metadata.
#[derive(Debug, PartialEq)]
pub struct CargoConfig {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
}

macro_rules! toml_val {
    ($toml:ident [ $key:expr ] . $cast:ident ( )) => ({
        $toml.get($key).and_then(|k| k.$cast()).ok_or(CargoInvalidError::Missing { key: $key })
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
            let mut f = File::open(path).map_err(|e| {
                    CargoParseError::Io {
                        src: path.to_string(),
                        err: e,
                    }
                })?;

            f.read_to_end(&mut buf)
                .map_err(|e| {
                    CargoParseError::Io {
                        src: path.to_string(),
                        err: e,
                    }
                })?;

            Cow::Owned(buf)
        }
        // Just use the buffer given
        ParseCargoArgs::FromBuf { buf } => buf,
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

            match toml_val!(toml["lib"].as_table()) {
                Ok(lib) => {
                    let tys: Vec<CargoCrateType> = toml_val!(lib["crate-type"].as_slice())?
                        .iter()
                        .filter_map(|t| t.as_str())
                        .map(|t| t.into())
                        .filter(|t| *t != CargoCrateType::Unknown)
                        .collect();

                    match tys.contains(&CargoCrateType::DyLib) {
                        true => Ok(tys),
                        _ => Err(CargoInvalidError::NotADyLib)
                    }
                },
                _ => Err(CargoInvalidError::NotADyLib)
            }?;

            Ok(CargoConfig {
                name: name,
                version: ver,
                authors: authors,
                description: desc,
            })
        }
        None => Err(CargoParseError::Toml { errs: parser.errors }),
    }
}

quick_error!{
    #[derive(Debug)]
    pub enum CargoInvalidError {
        /// A required value that wasn't in the config.
        ///
        /// This could be because it isn't present, in the wrong place,
        /// or has the wrong kind of value.
        Missing { key: &'static str } {
            display("The '{}' key is required, but wasn't found", key)
        }
        NotADyLib {
            display("The crate must include `dylib` in `lib.crate-type`")
        }
    }
}

quick_error!{
    /// An error encountered while parsing Cargo configuration.
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
        /// The cargo config is missing data.
        Invalid(err: CargoInvalidError) {
            cause(err)
            display("The config is invalid\nCaused by: {}", err)
            from()
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
        let args = ParseCargoArgs::FromFile { path: "tests/native/Cargo.toml" };

        parse_toml(args).unwrap();
    }

    #[test]
    fn parse_toml_from_buf() {
        let toml = r#"
            [package]
            name = "native"
            version = "0.1.0"
            authors = ["Somebody", "Somebody Else"]

            [lib]
            crate-type = ["rlib", "dylib"]
        "#;

        let args = ParseCargoArgs::FromBuf { buf: toml.as_bytes().into() };

        let toml = parse_toml(args).unwrap();

        let expected = CargoConfig {
            name: "native".into(),
            version: "0.1.0".into(),
            authors: vec!["Somebody".into(), "Somebody Else".into()],
            description: None,
        };

        assert_eq!(expected, toml);
    }

    macro_rules! test_invalid {
        ($input:expr, $err:pat) => ({
            let args = ParseCargoArgs::FromBuf { buf: $input.as_bytes().into() };

            let toml = parse_toml(args);

            match toml {
                Err($err) => (),
                r => panic!("{:?}", r)
            }
        })
    }

    #[test]
    fn parse_toml_missing_version() {
        test_invalid!(
            r#"
                [package]
                name = "native"
                authors = ["Somebody", "Somebody Else"]

                [lib]
                crate-type = ["rlib", "staticlib"]
            "#,
            CargoParseError::Invalid(CargoInvalidError::Missing { key: "version" })
        );
    }


    #[test]
    fn parse_toml_missing_name() {
        test_invalid!(
            r#"
                [package]
                version = "0.1.0"
                authors = ["Somebody", "Somebody Else"]

                [lib]
                crate-type = ["rlib", "staticlib"]
            "#,
            CargoParseError::Invalid(CargoInvalidError::Missing { key: "name" })
        );
    }

    #[test]
    fn parse_toml_not_a_dylib() {
        test_invalid!(
            r#"
                [package]
                name = "native"
                version = "0.1.0"
                authors = ["Somebody", "Somebody Else"]

                [lib]
                crate-type = ["rlib", "staticlib"]
            "#,
            CargoParseError::Invalid(CargoInvalidError::NotADyLib)
        );
    }

    #[test]
    fn parse_toml_missing_lib() {
        test_invalid!(
            r#"
                [package]
                name = "native"
                version = "0.1.0"
                authors = ["Somebody", "Somebody Else"]
            "#,
            CargoParseError::Invalid(CargoInvalidError::NotADyLib)
        );
    }
}
