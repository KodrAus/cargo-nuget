# `cargo-nuget`

Pack native Rust libraries as .NET Nuget packages.

## Build Status
Platform           | Channel | Status
------------------ | ------- | -------------
Linux (Debian x64) | Stable  | [![Build Status](https://travis-ci.org/KodrAus/nuget-rs.svg?branch=master)](https://travis-ci.org/KodrAus/nuget-rs)
Windows (MSVC x64) | Stable  | [![Build status](https://ci.appveyor.com/api/projects/status/v7mum8fgs5ho3oua?svg=true)](https://ci.appveyor.com/project/KodrAus/nuget-rs)

## Progress

- [x] Package builds for local target
- [ ] Release as cargo tool
- [ ] Package builds for cross-platform targets

## Installation

```shell
$ cargo install cargo-nuget
```

## Usage

Running `cargo-nuget` will attempt to pack a crate in the current directory as a `nupkg`:

```shell
$ cargo-nuget
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── your_crate.0.1.0.nupkg
├── src
│   └── lib.rs
└── target
```

For a complete set of commands:

```shell
cargo-nuget --help
```

### The process

Here's the basic workflow we want to support:

1. Write a Cargo-based Rust library
1. Populate your `Cargo.toml` crate metadata
1. Run `cargo-nuget` to run a `cargo build` and get a `nupkg` containing a dev build for your current platform
1. Reference your crate name as a dependency in your .NET project file
1. `DllImport` your crate name

Some additional options may be supplied:

```shell
$ cargo-nuget --test
$ cargo-nuget --cargo-dir=some-crate/path/
$ cargo-nuget --nupkg-dir=some-folder/nuget/
$ cargo-nuget --release
```

## About

This is a tool for packaging Rust libraries as a Nuget package for consuming in .NET. The basic idea is to:

- Use the native Rust target for a development build and write the package to some local feed
- Use `cross` for a cross-platform build to write multiple targets into the package for publishing

In general the tool should:

- Support typical Rust and .NET build pipelines
- Work

### Why use packages?

The new .NET Core tooling for packages is a big improvement over the old rubbish we had to deal with. I think it's possible to support development workflows using packages in .NET in a way we couldn't do before. Being able to referernce native assemblies using packages has the benefit of working the exact same way in dev as it would in the wild.