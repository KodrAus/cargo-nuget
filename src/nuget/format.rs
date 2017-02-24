//! Emit package metadata as `nuspec` XML.

use std::io::{Write, Error as IoError};
use std::borrow::Cow;
use xml::writer::{EventWriter, XmlEvent, Error as XmlError};
use xml::common::XmlVersion;
use xml::attribute::Attribute;
use xml::namespace::Namespace;

use super::Buf;

// TODO: May need "Microsoft.NETCore.Platforms": "*"

/// Args for building a `nuspec` metadata file.
#[derive(Debug, PartialEq)]
pub struct FormatNuspecArgs<'a> {
    pub id: Cow<'a, str>,
    pub version: Cow<'a, str>,
    pub authors: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
}

/// A formatted nuspec file.
#[derive(Debug, PartialEq)]
pub struct Nuspec {
    pub xml: Buf
}

/// Format the input as a `nuspec` xml buffer.
pub fn format_nuspec<'a>(args: FormatNuspecArgs<'a>) -> Result<Nuspec, FormatNuspecError> {
    let mut writer = EventWriter::new(Vec::new());

    // Write the version
    writer.write(XmlEvent::StartDocument {
            version: XmlVersion::Version10,
            encoding: None,
            standalone: None,
        })?;

    let pkg_attr = Attribute {
        name: "xmlns".into(),
        value: "http://schemas.microsoft.com/packaging/2012/06/nuspec.xsd",
    };

    elem(&mut writer, "package", &[pkg_attr], |ref mut writer| {
        elem(writer, "metadata", &[], |ref mut writer| {
            val(writer, "id", &args.id)?;
            val(writer, "version", &args.version)?;
            val(writer, "authors", &args.authors)?;

            if let Some(ref description) = args.description {
                val(writer, "description", &description)?;
            }

            Ok(())
        })
    })?;

    Ok(Nuspec { xml: writer.into_inner().into() })
}

fn elem<W, F>(writer: &mut EventWriter<W>,
              name: &str,
              attrs: &[Attribute],
              f: F)
              -> Result<(), FormatNuspecError>
    where W: Write,
          F: Fn(&mut EventWriter<W>) -> Result<(), FormatNuspecError>
{
    writer.write(XmlEvent::StartElement {
            name: name.into(),
            attributes: Cow::Borrowed(attrs),
            namespace: Cow::Owned(Namespace::empty()),
        })?;

    f(writer)?;

    writer.write(XmlEvent::EndElement { name: Some(name.into()) })?;

    Ok(())
}

fn val<W, V>(writer: &mut EventWriter<W>, name: &str, value: &V) -> Result<(), FormatNuspecError>
    where W: Write,
          V: AsRef<str>
{
    writer.write(XmlEvent::StartElement {
            name: name.into(),
            attributes: Cow::Borrowed(&[]),
            namespace: Cow::Owned(Namespace::empty()),
        })?;

    writer.write(XmlEvent::Characters(value.as_ref()))?;

    writer.write(XmlEvent::EndElement { name: Some(name.into()) })?;

    Ok(())
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
        Xml(err: XmlError) {
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
            description: Some(Cow::Borrowed("A description for this package")),
        };

        let nuspec = format_nuspec(args).unwrap();

        let expected = r#"<?xml version="1.0" encoding="UTF-8"?><package xmlns="http://schemas.microsoft.com/packaging/2012/06/nuspec.xsd"><metadata><id>native</id><version>0.1.0</version><authors>Someone</authors><description>A description for this package</description></metadata></package>"#;

        assert_eq!(expected, str::from_utf8(&nuspec.xml).unwrap());
    }
}
