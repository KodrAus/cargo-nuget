set -e

DOTNET_SDK_DOWNLOAD_URL=https://dotnetcli.blob.core.windows.net/dotnet/Sdk/$DOTNET_SDK_VERSION/dotnet-dev-osx-x64.$DOTNET_SDK_VERSION.tar.gz
RUST_ARCHIVE=rust-$RUST_VERSION-x86_64-apple-darwin.tar.gz
RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

# Install the .NET SDK
curl -L $DOTNET_SDK_DOWNLOAD_URL --output dotnet.tar.gz
mkdir $HOME/dotnet
tar zxf dotnet.tar.gz -C $HOME/dotnet
export PATH=$HOME/dotnet:$PATH

# Install Rust
curl -fsOSL $RUST_DOWNLOAD_URL
curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1
rm $RUST_ARCHIVE
./install.sh
