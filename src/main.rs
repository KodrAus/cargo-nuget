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
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

pub mod cargo;
pub mod nuget;
pub mod pack;
pub mod cross;
mod args;
mod logger;

use std::error::Error;
use std::process;

fn get_command(args: &clap::ArgMatches) -> Option<Result<(), Box<Error>>> {
    // Run pack command
    let pack_cmd = || args.subcommand_matches(args::PACK_CMD).map(pack::call);

    // Run cross command
    let cross_cmd = || args.subcommand_matches(args::CROSS_CMD).map(cross::call);
    
    pack_cmd().or_else(cross_cmd)
}

fn main() {
    logger::init();

    let args = args::app().get_matches();

    let mut result = BuildResult::default();

    if let Some(cmd) = get_command(&args) {
        result.ran = true;

        match cmd {
            Err(e) => {
                result.err = Some(e)
            },
            _ => ()
        }
    }

    match result {
        BuildResult { ran: false, .. } => {
            // print help and exit
            args::app().print_help().unwrap();
            println!("");
        },
        BuildResult { err: Some(e), .. } => {
            // print error and exit
            error!("{}", e);

            info!("\nThe build did not finish successfully");

            process::exit(1);
        },
        BuildResult { err: None, .. } => {
            // print success and exit
            info!("\nThe build finished successfully");

            process::exit(0);
        }
    }
}

#[derive(Default)]
struct BuildResult {
    ran: bool,
    err: Option<Box<Error>>,
}