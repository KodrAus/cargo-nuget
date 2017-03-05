# `nuget-rs`

> This is a WIP and the process described below doesn't work yet.

This is a tool for packaging Rust libraries as a Nuget package for consuming in .NET. The basic idea is to:

- Use the native Rust target for a development build and write the package to some local feed
- Use `cross` for a cross-platform build to write multiple targets into the package for publishing

In general the tool should:

- Support typical Rust and .NET build pipelines
- Work

## Why use packages?

The new .NET Core tooling for packages is a big improvement over the old rubbish we had to deal with. I think it's possible to support development workflows using packages in .NET in a way we couldn't do before. Being able to referernce native assemblies using packages has the benefit of working the exact same way in dev as it would in the wild.

## The process

Here's the basics workflow we want to support:

1. Write a Cargo-based Rust library
1. Populate your `Cargo.toml` crate metadata
1. Run `cargo nuget` to run a `cargo build` and get a `nupkg` containing a dev build for your current platform
1. Run `cargo nuget cross` to run `cargo cross` and get a `nupkg` containing builds for a couple of common platforms, built using `cross`

Some additional options may be supplied:

### Dev

```shell
$ cargo nuget
$ cargo nuget --cargo-dir=some-crate/path/
$ cargo nuget --nupkg-dir=target/nuget/
$ cargo nuget --release
```

### Release

```
$ cargo nuget cross
$ cargo nuget cross --release --target=win-x64 --target=osx
```

### In summary

Run `cargo nuget` with any of:

- `release` to run a release build
- `nupkg-dir` to specify the output path for the package

Additionally, when running `cargo nuget cross`:

- `target` to use the given set of platform targets instead of the default

Since `cross` is only supported on Linux, you should get a warning if you run `cargo nuget cross` on a different platform. The idea is that running `cargo nuget` with no parameters should just work everywhere.
