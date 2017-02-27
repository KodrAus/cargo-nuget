use clap::{App, Arg, ArgMatches};

pub const CARGO_WORK_DIR_ARG: &'static str = "cargo-dir";
pub const TEST_ARG: &'static str = "test";
pub const RELEASE_ARG: &'static str = "release";
pub const TARGET_ARG: &'static str = "target";
pub const NUGET_PATH_ARG: &'static str = "nuget-path";

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Nuget pack for Rust libraries").args(&[Arg::with_name(CARGO_WORK_DIR_ARG)
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
