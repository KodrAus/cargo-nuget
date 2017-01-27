# `nuget-rs`

> This is a WIP

This is a tool for packaging Rust libraries as a Nuget package for consuming in .NET. The basic idea is to:

- Use the native Rust target for a `dev` build and write the package to some local feed
- Use `cross` for a `pack` build to write multiple targets into the package for publishing

In general the tool should:

- Support typical Rust and .NET build pipelines
- Work

## Why use packages?

The new .NET Core tooling for packages is big improvement over the old rubbish we had to deal with. I think it's possible to support development workflows using packages in .NET in a way we couldn't do before. Being able to referernce native assemblies using packages has the benefit of working the exact same way in dev as it would in the wild.