echo "FROM microsoft/dotnet:$DOTNET_SDK_VERSION-sdk" > ci/linux/Dockerfile
cat ci/linux/Dockerfile.part >> ci/linux/Dockerfile
