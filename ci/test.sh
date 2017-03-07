set -e

dotnet --info
rustc -vV
cargo -vV

cargo test --verbose
cargo build

./target/debug/cargo-nuget --test --cargo-build-quiet --cargo-dir ./tests/native/ --nupkg-dir ./tests/feed/

dotnet restore tests/dotnet/dotnet.csproj --configfile ./Nuget.Config
dotnet run --project tests/dotnet/dotnet.csproj