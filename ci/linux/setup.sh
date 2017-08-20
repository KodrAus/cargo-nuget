echo "FROM microsoft/dotnet:$DOTNET_SDK_VERSION-sdk" > ci/linux/Dockerfile
cat Dockerfile.part >> ci/linux/Dockerfile
