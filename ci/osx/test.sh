set -e
ulimit -n 512

export PATH=./dotnet:$PATH

./ci/test.sh
