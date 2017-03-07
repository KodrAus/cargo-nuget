DOTNET_SDK_DOWNLOAD_URL=https://dotnetcli.blob.core.windows.net/dotnet/Sdk/$DOTNET_SDK_VERSION/dotnet-dev-osx-x64.$DOTNET_SDK_VERSION.tar.gz
RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/rust-$RUST_VERSION-x86_64-apple-darwin.tar.gz

# Install the .NET SDK
curl -L $DOTNET_SDK_DOWNLOAD_URL --output dotnet.tar.gz
mkdir $HOME/dotnet
tar zxf dotnet.tar.gz -C $HOME/dotnet
export PATH=$HOME/dotnet:$PATH

# Install Rust
curl -L $RUST_DOWNLOAD_URL --output rust.tar.gz
tar zxf rust.tar.gz
./install.sh