This is a folder the `dotnet` project will check for packages.

Run the following command in the root of this repo:

```shell
$ cargo run -- --cargo-dir ./tests/native/ --nupkg-dir ./tests/feed/
```

This will produce a package, like `native_test.0.0.1.nupkg` and save it in the `feed` folder. You can then restore packages for the `dotnet` project to use the new native binary:

```shell
$ rm -r ~/.nuget/packages/native_test/
$ cd tests/dotnet
$ dotnet restore --configfile ./Nuget.config
```

Manually clearing the package from `nuget`s cache is the only way to force it to reinstall if the version number hasn't changed. Maybe we can add a `clear-cache` command to `cargo-nuget`? Although that doesn't seem like its responsibility.