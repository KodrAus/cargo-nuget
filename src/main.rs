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
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

pub mod cargo;
pub mod nuget;
pub mod pack;
pub mod cross;
mod args;

use std::process;
use term_painter::ToStyle;
use term_painter::Color::*;

fn main() {
    pretty_env_logger::init().unwrap();

    let args = args::app().get_matches();

    // run pack command
    if let Some(args) = args.subcommand_matches(args::PACK_CMD) {
        match pack::call(args) {
            Ok(_) => {
                println!("{}", Green.paint("The build finished successfully"));
                process::exit(0);
            }
            Err(e) => {
                error!("{}", e);

                // TODO: Write to stderr
                println!("\n{}",
                         Red.bold().paint("The build did not finish successfully"));

                process::exit(1);
            }
        }
    }

    // run cross command
    if let Some(args) = args.subcommand_matches(args::CROSS_CMD) {
        match cross::call(args) {
            Ok(_) => {
                println!("{}", Green.paint("The build finished successfully"));
                process::exit(0);
            }
            Err(e) => {
                error!("{}", e);

                // TODO: Write to stderr
                println!("\n{}",
                         Red.bold().paint("The build did not finish successfully"));

                process::exit(1);
            }
        }
    }
    
    // print help and exit
    args::app().print_help().unwrap();
    println!("");
}
