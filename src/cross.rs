use std::error::Error;
use clap::ArgMatches;

use {cargo, nuget};

pub fn call(args: &ArgMatches) -> Result<(), Box<Error>> {
    let cargo_toml = pass!("reading cargo manifest" => args => cargo::parse_toml);

    let cargo_libs = pass!("building Rust lib" => (args, &cargo_toml) => |args| {
        let results = cargo::build_cross(args);
        println!("");

        results
    });

    let nuspec = pass!("building nuspec" => &cargo_toml => nuget::spec);

    let nupkg = pass!("building nupkg" => (&nuspec, &cargo_libs) => nuget::pack);

    pass!("saving nupkg" => (args, &nupkg) => nuget::save_nupkg);

    Ok(())
}
