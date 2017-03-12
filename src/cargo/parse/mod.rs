//! Parse metadata out of a `Cargo.toml`.

mod version;

use std::str::{self, Utf8Error};
use std::collections::BTreeMap;
use std::io::{Read, Error as IoError};
use std::borrow::Cow;
use std::fs::File;
use toml::{Parser, ParserError, Value};

/// Args for parsing a `Cargo.toml` package metadata file.
///
/// The source can either be a relative filepath or a byte buffer.
#[derive(Debug, PartialEq)]
pub struct CargoParseArgs<'a> {
    pub dev: bool,
    pub buf: CargoBufKind<'a>,
}

#[derive(Debug, PartialEq)]
pub enum CargoBufKind<'a> {
    FromFile { path: Cow<'a, str> },
    FromBuf { buf: Cow<'a, [u8]> },
}

/// The parsed `Cargo.toml` metadata.
#[derive(Debug, PartialEq)]
pub struct CargoConfig<'a> {
    pub name: Cow<'a, str>,
    pub version: Cow<'a, str>,
    pub authors: Vec<Cow<'a, str>>,
    pub description: Cow<'a, str>,
}

macro_rules! toml_val {
    ($toml:ident [ $key:expr ] . $cast:ident ( )) => ({
        $toml.get($key).and_then(|k| k.$cast()).ok_or(CargoInvalidError::Missing { key: $key })
    })
}

/// Parse `CargoConfig` from the given source.
pub fn parse_toml<'a>(args: CargoParseArgs<'a>) -> Result<CargoConfig<'a>, CargoParseError> {
    // Get a buffer to the toml file
    let buf = match args.buf {
        // Read the file to an owned buffer
        CargoBufKind::FromFile { path } => {
            let mut buf = Vec::new();
            let mut f = File::open(path.as_ref()).map_err(|e| {
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
        CargoBufKind::FromBuf { buf } => buf,
    };

    let utf8 = str::from_utf8(&buf)?;
    let mut parser = Parser::new(utf8);

    // Parse the toml config
    let mut parsed = match parser.parse() {
        Some(toml) => {
            ensure_crate_is_dylib(&toml).map_err(|_| CargoInvalidError::NotADyLib)?;

            let pkg = toml_val!(toml["package"].as_table())?;
            let name = toml_val!(pkg["name"].as_str())?.to_owned();
            let ver = toml_val!(pkg["version"].as_str())?.to_owned();
            let desc = toml_val!(pkg["description"].as_str())?.to_owned();
            let authors = toml_val!(pkg["authors"].as_slice())
                ?
                .iter()
                .filter_map(|a| a.as_str())
                .map(|a| {
                    let author = a.to_owned();
                    Cow::Owned(author)
                })
                .collect();

            Ok(CargoConfig {
                name: Cow::Owned(name),
                version: Cow::Owned(ver),
                authors: authors,
                description: Cow::Owned(desc),
            })
        }
        None => Err(CargoParseError::Toml { errs: parser.errors }),
    }?;

    if args.dev {
        let dev_version = version::make_dev_version(&parsed.version)?;
        parsed.version = dev_version.into();
    }

    Ok(parsed)
}

fn ensure_crate_is_dylib(toml: &BTreeMap<String, Value>) -> Result<(), CargoInvalidError> {
    let lib = toml_val!(toml["lib"].as_table())?;

    let is_dylib = toml_val!(lib["crate-type"].as_slice())
        ?
        .iter()
        .filter_map(|t| t.as_str())
        .any(|t| t == "dylib");

    match is_dylib {
        true => Ok(()),
        _ => Err(CargoInvalidError::NotADyLib),
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
        /// An error parsing the version.
        Version(err: version::CargoVersionError) {
            cause(err)
            display("Error parsing config\nCaused by: {}", err)
            from()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_toml_from_file() {
        let args = CargoParseArgs { 
            dev: false, 
            buf: CargoBufKind::FromFile { path: "tests/native/Cargo.toml".into() } 
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
            description = ""

            [lib]
            crate-type = ["rlib", "dylib"]
        "#;

        let args = CargoParseArgs { 
            dev: false, 
            buf: CargoBufKind::FromBuf { buf: toml.as_bytes().into() }
        };

        let toml = parse_toml(args).unwrap();

        let expected = CargoConfig {
            name: "native".into(),
            version: "0.1.0".into(),
            authors: vec!["Somebody".into(), "Somebody Else".into()],
            description: "".into(),
        };

        assert_eq!(expected, toml);
    }

    macro_rules! assert_inavlid {
        ($input:expr, $err:pat) => ({
            let args = CargoParseArgs { 
                dev: false, 
                buf: CargoBufKind::FromBuf { buf: $input.as_bytes().into() }
            };

            let toml = parse_toml(args);

            match toml {
                Err($err) => (),
                r => panic!("{:?}", r)
            }
        })
    }

    #[test]
    fn parse_toml_missing_version() {
        assert_inavlid!(r#"
                [package]
                name = "native"
                authors = ["Somebody", "Somebody Else"]

                [lib]
                crate-type = ["rlib", "dylib"]
            "#,
                      CargoParseError::Invalid(CargoInvalidError::Missing { key: "version" }));
    }


    #[test]
    fn parse_toml_missing_name() {
        assert_inavlid!(r#"
                [package]
                version = "0.1.0"
                authors = ["Somebody", "Somebody Else"]

                [lib]
                crate-type = ["rlib", "dylib"]
            "#,
                      CargoParseError::Invalid(CargoInvalidError::Missing { key: "name" }));
    }

    #[test]
    fn parse_toml_not_a_dylib() {
        assert_inavlid!(r#"
                [package]
                name = "native"
                version = "0.1.0"
                authors = ["Somebody", "Somebody Else"]

                [lib]
                crate-type = ["rlib", "staticlib"]
            "#,
                      CargoParseError::Invalid(CargoInvalidError::NotADyLib));
    }

    #[test]
    fn parse_toml_missing_lib() {
        assert_inavlid!(r#"
                [package]
                name = "native"
                version = "0.1.0"
                authors = ["Somebody", "Somebody Else"]
            "#,
                      CargoParseError::Invalid(CargoInvalidError::NotADyLib));
    }
}
