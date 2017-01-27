pub mod format;

use std::borrow::Cow;
use self::format::FormatNuspecArgs;
use cargo::parse::CargoConfig;

impl<'a> From<&'a CargoConfig> for FormatNuspecArgs<'a> {
	fn from(cargo: &'a CargoConfig) -> Self {
		FormatNuspecArgs {
			id: Cow::Borrowed(&cargo.name),
			version: Cow::Borrowed(&cargo.version),
			authors: Cow::Owned((&cargo.authors).join(", ")),
			description: cargo.description.clone().map(|d| Cow::Owned(d))
		}
	}
}