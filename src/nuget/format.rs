//! Emit package metadata as `nuspec` XML.

use std::io::{Write, Error as IoError};
use std::borrow::Cow;

use super::{xml, Buf};

const TARGET_FRAMEWORK: &'static str = ".NETStandard1.0";
const PLATFORM_PACKAGE_ID: &'static str = "Microsoft.NETCore.Platforms";
const PLATFORM_PACKAGE_VERSION: &'static str = "[1.0.1, )";

/// Args for building a `nuspec` metadata file.
#[derive(Debug, PartialEq)]
pub struct FormatNuspecArgs<'a> {
    pub id: Cow<'a, str>,
    pub version: Cow<'a, str>,
    pub authors: Cow<'a, str>,
    pub description: Cow<'a, str>,
}

/// A formatted nuspec file.
#[derive(Debug, PartialEq)]
pub struct Nuspec<'a> {
    pub id: Cow<'a, str>,
    pub version: Cow<'a, str>,
    pub xml: Buf,
}

/// Format the input as a `nuspec` xml buffer.
pub fn format_nuspec<'a>(args: FormatNuspecArgs<'a>) -> Result<Nuspec<'a>, FormatNuspecError> {
    let mut writer = xml::writer()?;

    let pkg_attr = xml::attr("xmlns",
                             "http://schemas.microsoft.com/packaging/2012/06/nuspec.xsd");

    xml::elem(&mut writer, "package", &[pkg_attr], |ref mut writer| {
        xml::elem(writer, "metadata", &[], |ref mut writer| {
            format_meta(&args, writer)?;
            format_dependencies(writer)
        })
    })?;

    Ok(Nuspec {
        id: args.id,
        version: args.version,
        xml: writer.into_inner().into(),
    })
}

/// Write basic nuspec metadata.
fn format_meta<'a>(args: &FormatNuspecArgs<'a>,
                   writer: &mut xml::Writer)
                   -> Result<(), xml::Error> {
    xml::val(writer, "id", &args.id)?;
    xml::val(writer, "version", &args.version)?;
    xml::val(writer, "authors", &args.authors)?;
    xml::val(writer, "description", &args.description)
}

/// Write package dependencies.
fn format_dependencies(writer: &mut xml::Writer) -> Result<(), xml::Error> {
    xml::elem(writer, "dependencies", &[], |ref mut writer| {
        let group_attr = xml::attr("targetFramework", TARGET_FRAMEWORK);

        xml::elem(writer, "group", &[group_attr], |ref mut writer| {
            let id_attr = xml::attr("id", PLATFORM_PACKAGE_ID);
            let ver_attr = xml::attr("version", PLATFORM_PACKAGE_VERSION);

            xml::elem(writer, "dependency", &[id_attr, ver_attr], |_| Ok(()))
        })
    })
}

quick_error!{
    /// An error encountered formatting a Nuspec.
    #[derive(Debug)]
    pub enum FormatNuspecError {
        /// An io-related error writing the nuspec.
        Io(err: IoError) {
            cause(err)
            display("Error writing nuget config\nCaused by: {}", err)
            from()
        }
        /// An xml formatting error.
        Xml(err: xml::Error) {
            display("Error writing nuget config\nCaused by: {}", err)
            from()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::str;
    use super::*;

    #[test]
    fn format_nuget() {
        let args = FormatNuspecArgs {
            id: Cow::Borrowed("native"),
            version: Cow::Borrowed("0.1.0"),
            authors: Cow::Borrowed("Someone"),
            description: Cow::Borrowed("A description for this package"),
        };

        let nuspec = format_nuspec(args).unwrap();

        let expected = r#"<?xml version="1.0" encoding="UTF-8"?><package xmlns="http://schemas.microsoft.com/packaging/2012/06/nuspec.xsd"><metadata><id>native</id><version>0.1.0</version><authors>Someone</authors><description>A description for this package</description></metadata></package>"#;

        assert_eq!(expected, str::from_utf8(&nuspec.xml).unwrap());
    }
}
