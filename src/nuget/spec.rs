//! Emit package metadata as `nuspec` XML.

use std::ops::Deref;
use std::io::Error as IoError;
use std::borrow::Cow;

use super::Buf;
use super::util::xml;

/// Nuget package dependency.
#[derive(Debug, PartialEq)]
pub struct NugetDependency<'a> {
    pub id: Cow<'a, str>,
    pub version: Cow<'a, str>,
}

/// A collection of nuget package dependencies.
#[derive(Debug, PartialEq)]
pub struct NugetDependencies<'a>(Vec<NugetDependency<'a>>);

/// The default set of dependencies includes `Microsoft.NETCore.Platforms`
/// which is needed to resolve the right native binary at runtime.
impl<'a> Default for NugetDependencies<'a> {
    fn default() -> Self {
        NugetDependencies(vec![
            NugetDependency {
                id: "Microsoft.NETCore.Platforms".into(),
                version: "[1.0.1, )".into(),
            }
        ])
    }
}

impl<'a> Deref for NugetDependencies<'a> {
    type Target = Vec<NugetDependency<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Args for building a `nuspec` metadata file.
#[derive(Debug, PartialEq)]
pub struct NugetSpecArgs<'a> {
    pub id: Cow<'a, str>,
    pub version: Cow<'a, str>,
    pub authors: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub dependencies: NugetDependencies<'a>,
}

/// A formatted nuspec file.
#[derive(Debug, PartialEq)]
pub struct Nuspec<'a> {
    pub id: Cow<'a, str>,
    pub version: Cow<'a, str>,
    pub xml: Buf,
}

/// Format the input as a `nuspec` xml buffer.
pub fn spec<'a>(args: NugetSpecArgs<'a>) -> Result<Nuspec<'a>, NugetSpecError> {
    let mut writer = xml::writer()?;

    let pkg_attr = xml::attr("xmlns",
                             "http://schemas.microsoft.com/packaging/2012/06/nuspec.xsd");

    xml::elem(&mut writer, "package", &[pkg_attr], |ref mut writer| {
        xml::elem(writer, "metadata", &[], |ref mut writer| {
            format_meta(&args, writer)?;
            format_dependencies(&args.dependencies, writer)
        })
    })?;

    Ok(Nuspec {
        id: args.id,
        version: args.version,
        xml: writer.into_inner().into(),
    })
}

/// Write basic nuspec metadata.
fn format_meta<'a>(args: &NugetSpecArgs<'a>,
                   writer: &mut xml::Writer)
                   -> Result<(), xml::Error> {
    xml::val(writer, "id", &args.id)?;
    xml::val(writer, "version", &args.version)?;
    xml::val(writer, "authors", &args.authors)?;
    xml::val(writer, "description", &args.description)
}

/// Write package dependencies.
fn format_dependencies<'a>(dependencies: &[NugetDependency<'a>], writer: &mut xml::Writer) -> Result<(), xml::Error> {
    xml::elem(writer, "dependencies", &[], |ref mut writer| {
        for dependency in dependencies {
            let id_attr = xml::attr("id", &dependency.id);
            let ver_attr = xml::attr("version", &dependency.version);

            xml::elem(writer, "dependency", &[id_attr, ver_attr], |_| Ok(()))?;
        }

        Ok(())
    })
}

quick_error!{
    /// An error encountered formatting a Nuspec.
    #[derive(Debug)]
    pub enum NugetSpecError {
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
        let args = NugetSpecArgs {
            id: Cow::Borrowed("native"),
            version: Cow::Borrowed("0.1.0"),
            authors: Cow::Borrowed("Someone"),
            description: Cow::Borrowed("A description for this package"),
        };

        let nuspec = spec(args).unwrap();

        let expected = r#"<?xml version="1.0" encoding="UTF-8"?><package xmlns="http://schemas.microsoft.com/packaging/2012/06/nuspec.xsd"><metadata><id>native</id><version>0.1.0</version><authors>Someone</authors><description>A description for this package</description></metadata></package>"#;

        assert_eq!(expected, str::from_utf8(&nuspec.xml).unwrap());
    }
}
