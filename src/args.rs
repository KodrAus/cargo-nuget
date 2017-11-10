use std::borrow::Cow;

use clap::{App, Arg, SubCommand};

pub const PACK_CMD: &'static str = "pack";
pub const CROSS_CMD: &'static str = "cross";

pub const CARGO_WORK_DIR_ARG: &'static str = "cargo-dir";
pub const CARGO_BUILD_QUIET_ARG: &'static str = "cargo-build-quiet";
pub const TARGETS_ARG: &'static str = "targets";
pub const TEST_ARG: &'static str = "test";
pub const RELEASE_ARG: &'static str = "release";
pub const NUPKG_DIR_ARG: &'static str = "nupkg-dir";

pub fn target_path_arg(target: CrossTarget) -> String {
    format!("{}-path", target.rid())
}

struct PartialArg {
    name: String,
    long: String,
    help: String,
}

lazy_static! {
    static ref TARGET_PATHS: Vec<PartialArg> = {
        let archs = vec![
            Arch::x86,
            Arch::x64
        ];

        archs.into_iter().flat_map(|arch| {
            vec![
                CrossTarget::Windows(arch),
                CrossTarget::MacOS(arch),
                CrossTarget::Linux(arch),
            ]
        })
        .map(|target| PartialArg {
            name: target_path_arg(target),
            long: target_path_arg(target),
            help: format!("a specific path to the output for the {} target", target.rid())
        })
        .collect()
    };
}

pub fn app<'a, 'b>() -> App<'a, 'b> {
    let local_args = vec![
        Arg::with_name(CARGO_WORK_DIR_ARG)
            .long(CARGO_WORK_DIR_ARG)
            .takes_value(true)
            .help("path to the Rust crate"),
        Arg::with_name(CARGO_BUILD_QUIET_ARG)
            .short("q")
            .long(CARGO_BUILD_QUIET_ARG)
            .help("don't print output from cargo commands"),
        Arg::with_name(TEST_ARG)
            .short("t")
            .long(TEST_ARG)
            .help("run cargo tests"),
        Arg::with_name(RELEASE_ARG)
            .short("r")
            .long(RELEASE_ARG)
            .help("run an optimised build"),
        Arg::with_name(NUPKG_DIR_ARG)
            .long(NUPKG_DIR_ARG)
            .takes_value(true)
            .help("path to save the nupkg")];

    let path_args = TARGET_PATHS.iter().map(|arg| {
        Arg::with_name(&arg.name)
            .long(&arg.long)
            .takes_value(true)
            .help(&arg.help)
    });

    let mut cross_args = vec![
        Arg::with_name(CARGO_WORK_DIR_ARG)
            .long(CARGO_WORK_DIR_ARG)
            .takes_value(true)
            .help("path to the Rust crate"),
        Arg::with_name(TARGETS_ARG)
            .long(TARGETS_ARG)
            .takes_value(true)
            .required(true)
            .multiple(true)
            .help("set of dotnet rids to include"),
        Arg::with_name(CARGO_BUILD_QUIET_ARG)
            .short("q")
            .long(CARGO_BUILD_QUIET_ARG)
            .help("don't print output from cargo commands"),
        Arg::with_name(TEST_ARG)
            .short("t")
            .long(TEST_ARG)
            .help("run cargo tests"),
        Arg::with_name(RELEASE_ARG)
            .short("r")
            .long(RELEASE_ARG)
            .help("run an optimised build"),
        Arg::with_name(NUPKG_DIR_ARG)
            .long(NUPKG_DIR_ARG)
            .takes_value(true)
            .help("path to save the nupkg")];
    
    cross_args.extend(path_args);
    
    App::new("cargo-nuget")
        .version(crate_version!())
        .subcommand(SubCommand::with_name(PACK_CMD)
            .about("Pack a Rust library as a Nuget package for local development")
            .args(&local_args))
        .subcommand(SubCommand::with_name(CROSS_CMD)
            .about("Pack a Rust library as a Nuget package for cross-platform distribution")
            .args(&cross_args))
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

    pub fn rid(&self) -> Cow<'static, str> {
        match self.cross() {
            Some(target) => target.rid(),
            _ => "any".into(),
        }
    }

    pub fn from_rid(rid: &str) -> Self {
        match CrossTarget::from_rid(rid) {
            Some(target) => Target::Cross(target),
            None => Target::Unknown,
        }
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

    pub fn rid(&self) -> Cow<'static, str> {
        match *self {
            CrossTarget::Windows(arch) => rid("win", arch.rid()),
            CrossTarget::MacOS(arch) => rid("osx", arch.rid()),
            CrossTarget::Linux(arch) => rid("linux", arch.rid()),
        }
    }

    pub fn from_rid(rid: &str) -> Option<Self> {
        let mut parts = rid.split("-");

        let platform = parts.next();
        let arch = parts.next().and_then(Arch::from_rid);
        
        platform
            .and_then(|platform| arch.map(|arch| (platform, arch)))
            .and_then(|(platform, arch)| {
                match platform {
                    "win" => Some(CrossTarget::Windows(arch)),
                    "osx" => Some(CrossTarget::MacOS(arch)),
                    "linux" => Some(CrossTarget::Linux(arch)),
                    _ => None,
                }
            })
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

    pub fn rid(&self) -> &'static str {
        match *self {
            Arch::x86 => "x86",
            Arch::x64 => "x64",
        }
    }

    pub fn from_rid(rid: &str) -> Option<Self> {
        match rid {
            "x86" => Some(Arch::x86),
            "x64" => Some(Arch::x64),
            _ => None
        }
    }
}

fn rid(target: &'static str, arch: &'static str) -> Cow<'static, str> {
    format!("{}-{}", target, arch).into()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_rid() {
        unimplemented!("malformed rid")
    }

    #[test]
    fn windows_x86_rid() {
        unimplemented!("round trip to and from rid")
    }

    #[test]
    fn windows_x64_rid() {
        unimplemented!("round trip to and from rid")
    }

    #[test]
    fn osx_x86_rid() {
        unimplemented!("round trip to and from rid")
    }

    #[test]
    fn osx_x64_rid() {
        unimplemented!("round trip to and from rid")
    }

    #[test]
    fn linux_x86_rid() {
        unimplemented!("round trip to and from rid")
    }

    #[test]
    fn linux_x64_rid() {
        unimplemented!("round trip to and from rid")
    }
}
