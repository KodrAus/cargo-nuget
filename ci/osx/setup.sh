set -e

DOTNET_SDK_DOWNLOAD_URL=https://dotnetcli.blob.core.windows.net/dotnet/Sdk/$DOTNET_SDK_VERSION/dotnet-sdk-$DOTNET_SDK_VERSION-osx-x64.tar.gz

RUST_ARCHIVE=rust-$RUST_VERSION-x86_64-apple-darwin
RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE.tar.gz

# Install the .NET SDK
mkdir ./build/
curl -SL $DOTNET_SDK_DOWNLOAD_URL --output build/dotnet.tar.gz
tar -zxf build/dotnet.tar.gz -C $HOME/dotnet

ln -s /usr/local/opt/openssl/lib/libcrypto.1.0.0.dylib /usr/local/lib/
ln -s /usr/local/opt/openssl/lib/libssl.1.0.0.dylib /usr/local/lib/

# Install Rust
curl -L $RUST_DOWNLOAD_URL --output rust.tar.gz
tar zxf rust.tar.gz
./$RUST_ARCHIVE/install.sh
