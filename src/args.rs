use clap::{App, Arg, SubCommand};

pub const PACK_CMD: &'static str = "pack";

pub const CARGO_WORK_DIR_ARG: &'static str = "cargo-dir";
pub const CARGO_BUILD_QUIET: &'static str = "cargo-build-quiet";
pub const TEST_ARG: &'static str = "test";
pub const RELEASE_ARG: &'static str = "release";
pub const NUPKG_DIR_ARG: &'static str = "nupkg-dir";

pub fn app<'a, 'b>() -> App<'a, 'b> {
    let args = vec![Arg::with_name(CARGO_WORK_DIR_ARG)
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
                    .help("path to save the nupkg")];
    
    App::new("cargo-nuget")
        .version(crate_version!())
        .subcommand(SubCommand::with_name(PACK_CMD)
        .about("Pack a Rust library as a Nuget package for local development")
        .args(&args))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Build {
    Local {
        source: Source,
    },
    Cross {
        target: CrossTarget,
        source: Source,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    Build {
        action: Action,
        profile: Profile,
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Action {
    Build,
    Test,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Profile {
    Debug,
    Release,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Target {
    Local,
    Unknown,
    Cross(CrossTarget),
}

impl Target {
    pub fn cross(&self) -> Option<CrossTarget> {
        match *self {
            Target::Local => CrossTarget::local(),
            Target::Unknown => None,
            Target::Cross(target) => Some(target),
        }
    }

    pub fn is_unknown(&self) -> bool {
        self.cross().is_none()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum CrossTarget {
    Windows(Arch),
    Linux(Arch),
    MacOS(Arch),
}

impl CrossTarget {
    pub fn local() -> Option<Self> {
        local_target()
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Arch {
    x64,
    x86,
}

impl Arch {
    pub fn local() -> Option<Self> {
        local_arch()
    }
}

#[cfg(target_arch = "x86")]
fn local_arch() -> Option<Arch> { Some(Arch::x86) }
#[cfg(target_arch = "x86_64")]
fn local_arch() -> Option<Arch> { Some(Arch::x64) }

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn local_arch() -> Option<Arch> { None }

#[cfg(windows)]
fn local_target() -> Option<CrossTarget> {
    local_arch().map(|arch| CrossTarget::Windows(arch))
}
#[cfg(target_os = "macos")]
fn local_target() -> Option<CrossTarget> {
    local_arch().map(|arch| CrossTarget::MacOs(arch))
}
#[cfg(target_os = "linux")]
fn local_target() -> Option<CrossTarget> {
    local_arch().map(|arch| CrossTarget::Linux(arch))
}

#[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
fn local_target() -> Option<CrossTarget> {
    None
}
