docker build -t nugetrs/build:latest ci/linux/
docker run -v $(pwd):/src -w /src nugetrs/build:latest /bin/bash -c './ci/test.sh'
