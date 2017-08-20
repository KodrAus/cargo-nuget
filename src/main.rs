#![deny(warnings)]

#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate clap;
extern crate term_painter;
extern crate xml;
extern crate zip;
extern crate toml;
extern crate semver;
extern crate chrono;

#[macro_use]
mod macros;

pub mod cargo;
pub mod nuget;
mod args;

use std::error::Error;

use term_painter::ToStyle;
use term_painter::Color::*;

use clap::ArgMatches;

fn main() {
    let args = args::app().get_matches();

    // run pack command
    if let Some(args) = args.subcommand_matches(args::PACK_CMD) {
        match pack(args) {
            Ok(_) => {
                println!("{}", Green.paint("The build finished successfully"));
            }
            Err(e) => {
                println!("{}", Red.paint(e));
                println!("\n{}",
                         Red.bold().paint("The build did not finish successfully"));
            }
        }
    }
    // print help and exit
    else {
        args::app().print_help().unwrap();
        println!("");
    }
}

fn pack(args: &ArgMatches) -> Result<(), Box<Error>> {
    let mut cargo_toml = pass!("reading cargo manifest" => args => cargo::parse_toml);

    let local = pass!("adding local version tag" => &cargo_toml => cargo::local_version_tag);

    cargo_toml.version = local.version;

    let cargo_lib = pass!("building Rust lib" => (args, &cargo_toml) => |args| {
        let result = cargo::build_lib(args);
        println!("");

        result
    });

    let nuspec = pass!("building nuspec" => &cargo_toml => nuget::spec);

    let nupkg = pass!("building nupkg" => (&nuspec, &cargo_lib) => nuget::pack);

    pass!("saving nupkg" => (args, &nupkg) => nuget::save_nupkg);

    Ok(())
}
