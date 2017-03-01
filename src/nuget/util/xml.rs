//! Utilities for writing XML.

use std::borrow::Cow;
use std::io::Write;

use xml::writer::{EventWriter, XmlEvent};
use xml::common::XmlVersion;
use xml::name::Name;
use xml::attribute::Attribute;
use xml::namespace::Namespace;

pub use xml::writer::Error;
pub type Writer = EventWriter<Vec<u8>>;

pub fn writer() -> Result<Writer, Error> {
    let mut writer = Writer::new(Vec::new());

    // Write the version
    writer.write(XmlEvent::StartDocument {
            version: XmlVersion::Version10,
            encoding: None,
            standalone: None,
        })?;

    Ok(writer)
}

pub fn attr<'a, K>(id: K, value: &'a str) -> Attribute<'a>
    where K: Into<Name<'a>>
{
    Attribute {
        name: id.into(),
        value: value,
    }
}

pub fn elem<W, F>(writer: &mut EventWriter<W>,
                  name: &str,
                  attrs: &[Attribute],
                  f: F)
                  -> Result<(), Error>
    where W: Write,
          F: Fn(&mut EventWriter<W>) -> Result<(), Error>
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

pub fn val<W, V>(writer: &mut EventWriter<W>, name: &str, value: &V) -> Result<(), Error>
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
