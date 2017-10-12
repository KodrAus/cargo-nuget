// #![deny(warnings)]

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
pub mod pack;
mod args;

use term_painter::ToStyle;
use term_painter::Color::*;

fn main() {
    let args = args::app().get_matches();

    // run pack command
    if let Some(args) = args.subcommand_matches(args::PACK_CMD) {
        match pack::call(args) {
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
