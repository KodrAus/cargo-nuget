# `cargo-nuget` [![Latest Version](https://img.shields.io/crates/v/cargo-nuget.svg)](https://crates.io/crates/cargo-nuget)

Pack native Rust libraries as .NET Nuget packages.

## Build Status

Platform                             | Rust Version      | DotNet SDK Version | Status
------------------------------------ | ----------------- | ------------------ | ---------------
Linux (Debian x64) / OSX (10.12 x64) | Stable (1.18.0)   | 2.0.0              | [![Build Status](https://travis-ci.org/KodrAus/cargo-nuget.svg?branch=master)](https://travis-ci.org/KodrAus/cargo-nuget)
Windows (MSVC x64)                   | Stable (1.18.0)   | 2.0.0              | [![Build status](https://ci.appveyor.com/api/projects/status/v7mum8fgs5ho3oua?svg=true)](https://ci.appveyor.com/project/KodrAus/nuget-rs)

## Progress

- [x] Package builds for local target
- [x] Release as cargo tool
- [x] Package builds for cross-platform targets _(limited to pre-built cross-platform libs)_

## Installation

```shell
$ cargo install cargo-nuget
```

## Usage

See [a complete example](https://github.com/KodrAus/cargo-nuget/tree/master/tests).

Running `cargo-nuget pack` will attempt to pack a local build of the crate in the current directory as a `nupkg`:

```shell
$ cargo-nuget pack
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── your_crate.0.1.0-dev.1489461345.nupkg
├── src
│   └── lib.rs
└── target
```

Running `cargo-nuget cross` will attempt to pack a cross-platform build of the crate in the current directory as a `nupkg`:

```shell
$ cargo-nuget cross --targets win-x64 linux-x64 osx-x64 --win-x64-path ./target/x86_64-pc-windows-gnu/debug/your_crate.dll --linux-x64-path ./target/x86_64-unknown-linux-gnu/debug/libyour_crate.so --osx-x64-path ./target/x86_64-apple-darwin/debug/libyour_crate.dylib
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── your_crate.0.1.0.nupkg
├── src
│   └── lib.rs
└── target
```

The `cross` command requires the relevant targets are listed along with paths to pre-built libraries for each platform.
This restriction should be removed in the future.

For a complete set of commands:

```shell
$ cargo-nuget --help
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
$ cargo-nuget pack --test
$ cargo-nuget pack --cargo-dir=some-crate/path/
$ cargo-nuget pack --nupkg-dir=some-folder/nuget/
$ cargo-nuget pack --release
```

## About

This is a tool for packaging Rust libraries as a Nuget package for consuming in .NET. The basic idea is to use the native Rust target for a development build and write the package to some local feed

In general the tool should:

- Support typical Rust and .NET build pipelines
- Work

### Why use packages?

The new .NET Core tooling for packages is a big improvement over the old rubbish we had to deal with. I think it's possible to support development workflows using packages in .NET in a way we couldn't do before. Being able to referernce native assemblies using packages has the benefit of working the exact same way in dev as it would in the wild.
