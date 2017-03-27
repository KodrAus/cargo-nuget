set -e

DOTNET_SDK_DOWNLOAD_URL=https://raw.githubusercontent.com/dotnet/cli/rel/1.0.0/scripts/obtain/dotnet-install.sh

RUST_ARCHIVE=rust-$RUST_VERSION-x86_64-apple-darwin
RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE.tar.gz

# Install the .NET SDK
mkdir ./build/
curl -L $DOTNET_SDK_DOWNLOAD_URL --output dotnet.tar.gz
./build/installcli.sh -InstallDir $HOME/dotnet -NoPath -Version $DOTNET_SDK_VERSION

ln -s /usr/local/opt/openssl/lib/libcrypto.1.0.0.dylib /usr/local/lib/
ln -s /usr/local/opt/openssl/lib/libssl.1.0.0.dylib /usr/local/lib/

# Install Rust
curl -L $RUST_DOWNLOAD_URL --output rust.tar.gz
tar zxf rust.tar.gz
./$RUST_ARCHIVE/install.sh
