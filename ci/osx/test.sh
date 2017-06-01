set -e
ulimit -n 512

export PATH=$HOME/dotnet:$PATH

./ci/test.sh
