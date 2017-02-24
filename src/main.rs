#[macro_use]
extern crate quick_error;
extern crate clap;
extern crate term_painter;
extern crate xml;
extern crate zip;
extern crate toml;

pub mod cargo;
pub mod nuget;

use std::error::Error;

use term_painter::ToStyle;
use term_painter::Color::*;

use clap::{App, Arg, ArgMatches};

const CARGO_WORK_DIR_ARG: &'static str = "cargo-dir"; 
const TEST_ARG: &'static str = "test";
const RELEASE_ARG: &'static str = "release";
const TARGET_ARG: &'static str = "target";
const NUGET_PATH_ARG: &'static str = "nuget-path";

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Nuget pack for Rust libraries").args(
        &[Arg::with_name(CARGO_WORK_DIR_ARG)
              .long("cargo-dir")
              .takes_value(true)
              .help("path to the Rust crate"),
          Arg::with_name(TEST_ARG)
              .short("t")
              .long("test")
              .help("run cargo and dotnet tests"),
          Arg::with_name(RELEASE_ARG)
              .short("r")
              .long("release")
              .help("run an optimised build"),
          Arg::with_name(TARGET_ARG)
              .long("target")
              .multiple(true)
              .takes_value(true)
              .help("a platform to target"),
          Arg::with_name(NUGET_PATH_ARG)
              .long("nuget-path")
              .takes_value(true)
              .help("path to save the nupkg")])
}

fn main() {
    let matches = app().get_matches();

    match build(matches) {
        Ok(_) => {
            println!("{}", Green.paint("The build finished successfully"));
        }
        Err(e) => {
            println!("{}", Red.paint(e));
            println!("\n{}", Red.bold().paint("The build did not finish successfully"));
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
