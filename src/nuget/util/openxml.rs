//! OpenXML specific files.

use std::path::{Path, PathBuf};

use super::xml;

pub fn content_types() -> Result<(PathBuf, Vec<u8>), xml::Error> {
    let mut writer = xml::writer()?;

    let ns = xml::attr("xmlns",
                       "http://schemas.openxmlformats.org/package/2006/content-types");

    xml::elem(&mut writer, "Types", &[ns], |ref mut writer| {
        fn default(writer: &mut xml::Writer,
                   extension: &str,
                   content_type: &str)
                   -> Result<(), xml::Error> {
            let extension = xml::attr("Extension", extension);
            let content_type = xml::attr("ContentType", content_type);

            xml::elem(writer, "Default", &[extension, content_type], |_| Ok(()))
        }

        let types = [("rels", "application/vnd.openxmlformats-package.relationships+xml"),
                     ("txt", "application/octet"),
                     ("dll", "application/octet"),
                     ("dylib", "application/octet"),
                     ("so", "application/octet"),
                     ("nuspec", "application/octet")];

        for &(extension, content_type) in &types {
            default(writer, extension, content_type)?;
        }

        Ok(())
    })?;

    let mut path = PathBuf::new();
    path.set_file_name("[Content_Types]");
    path.set_extension("xml");

    Ok((path, writer.into_inner()))
}

pub fn relationships<P>(nuspec_path: P) -> Result<(PathBuf, Vec<u8>), xml::Error> 
    where P: AsRef<Path>
{
    let nuspec_path = nuspec_path.as_ref();

    let mut writer = xml::writer()?;

    let ns = xml::attr("xmlns",
                       "http://schemas.openxmlformats.org/package/2006/relationships");

    xml::elem(&mut writer, "Relationships", &[ns], |ref mut writer| {
        let ty = xml::attr("Type",
                           "http://schemas.microsoft.com/packaging/2010/07/manifest");

        let target = format!("/{}", nuspec_path.to_string_lossy());
        let target = xml::attr("Target", &target);

        xml::elem(writer, "Relationship", &[ty, target], |_| Ok(()))
    })?;

    let mut path = PathBuf::new();
    path.push("_rels");
    path.push(".rels");

    Ok((path, writer.into_inner()))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn rels_file() {
        let (path, content) = relationships("some/path/spec.nuspec").unwrap();

        let expected = br#"
            <?xml version="1.0" encoding="UTF-8"?>
            <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                <Relationship Type="http://schemas.microsoft.com/packaging/2010/07/manifest" Target="/some/path/spec.nuspec" />
            </Relationships>
        "#;

        assert_eq!(PathBuf::from("_rels/.rels"), path);
        assert_eq_no_ws!(expected, &content);
    }

    #[test]
    fn content_types_file() {
        let (path, content) = content_types().unwrap();

        let expected = br#"
            <?xml version="1.0" encoding="UTF-8"?>
            <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml" />
                <Default Extension="txt" ContentType="application/octet" />
                <Default Extension="dll" ContentType="application/octet" />
                <Default Extension="dylib" ContentType="application/octet" />
                <Default Extension="so" ContentType="application/octet" />
                <Default Extension="nuspec" ContentType="application/octet" />
            </Types>
        "#;

        assert_eq!(PathBuf::from("[Content_Types].xml"), path);
        assert_eq_no_ws!(expected, &content);
    }
}