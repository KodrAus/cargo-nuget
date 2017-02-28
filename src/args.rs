use clap::{App, Arg};

pub const CARGO_WORK_DIR_ARG: &'static str = "cargo-dir";
pub const TEST_ARG: &'static str = "test";
pub const RELEASE_ARG: &'static str = "release";
pub const TARGET_ARG: &'static str = "target";
pub const NUPKG_DIR_ARG: &'static str = "nupkg-dir";

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Nuget pack for Rust libraries").args(&[Arg::with_name(CARGO_WORK_DIR_ARG)
                                                         .long(CARGO_WORK_DIR_ARG)
                                                         .takes_value(true)
                                                         .help("path to the Rust crate"),
                                                     Arg::with_name(TEST_ARG)
                                                         .short("t")
                                                         .long(TEST_ARG)
                                                         .help("run cargo and dotnet tests"),
                                                     Arg::with_name(RELEASE_ARG)
                                                         .short("r")
                                                         .long(RELEASE_ARG)
                                                         .help("run an optimised build"),
                                                     Arg::with_name(TARGET_ARG)
                                                         .long(TARGET_ARG)
                                                         .multiple(true)
                                                         .takes_value(true)
                                                         .help("a platform to target"),
                                                     Arg::with_name(NUPKG_DIR_ARG)
                                                         .long(NUPKG_DIR_ARG)
                                                         .takes_value(true)
                                                         .help("path to save the nupkg")])
}
