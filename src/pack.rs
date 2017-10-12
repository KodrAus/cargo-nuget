use std::error::Error;
use clap::ArgMatches;

use {cargo, nuget};

pub fn call(args: &ArgMatches) -> Result<(), Box<Error>> {
    let mut cargo_toml = pass!("reading cargo manifest" => args => cargo::parse_toml);

    let local = pass!("adding local version tag" => &cargo_toml => cargo::local_version_tag);

    cargo_toml.version = local.version;

    let cargo_libs = pass!("building Rust lib" => (args, &cargo_toml) => |args| {
        let result = cargo::build_local(args);
        println!("");

        result.map(|result| vec![result])
    });

    let nuspec = pass!("building nuspec" => &cargo_toml => nuget::spec);

    let nupkg = pass!("building nupkg" => (&nuspec, &cargo_libs) => nuget::pack);

    pass!("saving nupkg" => (args, &nupkg) => nuget::save_nupkg);

    Ok(())
}
