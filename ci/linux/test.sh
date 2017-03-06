docker build -t nugetrs/build:latest ci/linux/
docker run nugetrs/build:latest /bin/bash -v $(pwd):/src -c 'cd /src && ./ci/test.sh'
