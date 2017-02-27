#[macro_use]
extern crate quick_error;
extern crate clap;
extern crate term_painter;
extern crate xml;
extern crate zip;
extern crate toml;

pub mod cargo;
pub mod nuget;
mod args;

use std::error::Error;

use term_painter::ToStyle;
use term_painter::Color::*;

use clap::{App, Arg, ArgMatches};

fn main() {
    let args = args::app().get_matches();

    match build(args) {
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

macro_rules! pass {
    ($line:expr => $args:expr => $pass:expr) => ({
        use term_painter::ToStyle;
        use term_painter::Color::*;

        let args = $args.into();

        println!("{}\n\n{}", $line, Cyan.bold().paint(format!("input: {:?}\n", args)));

        let result = $pass(args)?;

        println!("{}\n", Cyan.bold().paint(format!("output: {:?}", result)));

        result
    })
}

fn build(args: ArgMatches) -> Result<(), Box<Error>> {
    let cargo_toml = pass!("reading cargo manifest" => &args => cargo::parse_toml);

    let cargo_lib = pass!("building Rust lib" => (&args, &cargo_toml) => |args| {
        let result = cargo::build_lib(args);
        println!();

        result
    });

    let nuspec = pass!("building nuspec" => &cargo_toml => nuget::format_nuspec);

    let _nupkg = pass!("building nupkg" => (&nuspec, &cargo_lib) => nuget::pack);

    // TODO: Write nupkg to file

    Ok(())
}
