use std::io::Error as IoError;
use xml::writer::{EventWriter, EmitterConfig, XmlEvent, Error as XmlError};
/// Args for building a `nuspec` metadata file.
#[derive(Debug, PartialEq)]
pub struct FormatNuspecArgs<'a> {
	pub name: &'a str,
	pub version: &'a str,
	pub author: &'a str,
	pub description: Option<&'a str>
}

/// Format the input as a `nuspec` xml buffer.
pub fn format_nuspec<'a, T, R>(args: T) -> Result<Vec<u8>, FormatNuspecError>
	where T: Into<FormatNuspecArgs<'a>>
{
	let args = args.into();

	unimplemented!();
}

quick_error!{
	#[derive(Debug)]
	pub enum FormatNuspecError {
		Io(err: IoError) {
			cause(err)
			display("Error writing nuget config\nCaused by: {}", err)
			from()
		}
		Xml(err: XmlError) {
			display("Error writing nuget config\nCaused by: {}", err)
			from()
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn format_nuget() {
		unimplemented!();
	}
}