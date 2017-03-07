This directory contains an example project using a Rust library packed by `cargo-nuget` and consumed in C#. It's structed as follows:

```
.
├── dotnet
│   ├── dotnet.csproj
│   ├── Nuget.Config
│   └── Program.cs
├── feed
├── native
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── README.md
```

- `./dotnet`: the C# project
- `./feed`: where native packages will be published
- `./native`: the Rust project 

## Pack the Rust library

From this directory, run:

```shell
$ cargo-nuget --cargo-dir ./native --nupkg-dir ./feed
```

This will produce a package, like `native_test.0.0.1.nupkg` and save it in the `feed` folder. 

## Use the package in C#

You can then restore packages for the `dotnet` project to use the new native binary:

```shell
$ dotnet restore ./dotnet/dotnet.csproj --configfile ./Nuget.Config
```

> **NOTE**: You'll probably have to clear nuget's cached `native_test` package if you change the binary but not the version number, or restore won't pick up your changes.