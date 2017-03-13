This directory contains an example project using a Rust library packed by `cargo-nuget` and consumed in C#. It's structured as follows:

```
.
├── dotnet
│   ├── dotnet.csproj
│   ├── Nuget.Config
│   └── Program.cs
├── feed
├── native
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
$ cargo-nuget pack --cargo-dir ./native --nupkg-dir ./feed
```

This will produce a package, like `native_test.0.0.1-dev.123456789.nupkg` and save it in the `feed` folder. 

## Use the package in C#

You can then restore packages for the `dotnet` project to use the new native binary:

```shell
$ dotnet restore ./dotnet/dotnet.csproj --configfile ./Nuget.Config
```
