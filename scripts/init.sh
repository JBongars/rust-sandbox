#!/bin/sh

set -eu
set -o pipefail

# install rustfmt
rustup component add rustfmt
