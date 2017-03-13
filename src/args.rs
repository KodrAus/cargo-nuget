use clap::{App, Arg, SubCommand};

pub const PACK_CMD: &'static str = "pack";

pub const CARGO_WORK_DIR_ARG: &'static str = "cargo-dir";
pub const CARGO_BUILD_QUIET: &'static str = "cargo-build-quiet";
pub const TEST_ARG: &'static str = "test";
pub const RELEASE_ARG: &'static str = "release";
pub const NUPKG_DIR_ARG: &'static str = "nupkg-dir";

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("cargo-nuget")
        .subcommand(SubCommand::with_name(PACK_CMD)
            .about("Pack a Rust library as a Nuget package for local development") 
            .args(&[Arg::with_name(CARGO_WORK_DIR_ARG)
                 .long(CARGO_WORK_DIR_ARG)
                 .takes_value(true)
                 .help("path to the Rust crate"),
             Arg::with_name(CARGO_BUILD_QUIET)
                 .short("q")
                 .long(CARGO_BUILD_QUIET)
                 .help("don't print output from cargo commands"),
             Arg::with_name(TEST_ARG)
                 .short("t")
                 .long(TEST_ARG)
                 .help("run cargo and dotnet tests"),
             Arg::with_name(RELEASE_ARG)
                 .short("r")
                 .long(RELEASE_ARG)
                 .help("run an optimised build"),
             Arg::with_name(NUPKG_DIR_ARG)
                 .long(NUPKG_DIR_ARG)
                 .takes_value(true)
                 .help("path to save the nupkg")]))
}
