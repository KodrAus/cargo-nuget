set -e

docker build --build-arg RUST_VERSION=$RUST_VERSION -t nugetrs/build:latest ci/linux/
docker run -v $(pwd):/src -w /src nugetrs/build:latest /bin/bash -c './ci/test.sh'
