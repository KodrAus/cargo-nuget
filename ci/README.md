This folder contains some helper scripts for CI. The main script for each environment can be found at `${OS}/test.${EXT}`. Shared scripts live in the root `ci` directory.

Scripts are all expected to be run from the repository root.

You shouldn't need to run the `setup` script locally, that's just for the CI environment, but the `test` and `cleanup` scripts should be usable anywhere.