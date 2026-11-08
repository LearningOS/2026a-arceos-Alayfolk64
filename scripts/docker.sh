#!/bin/sh
set -eu

cd "$(dirname "$0")/.."
image=docker.io/1301182193/arceos-env@sha256:ae65065f36a71631a698f5b6dac04bdadda5ab6f9ebc77c032db15d126914bde
exec docker run --pull=missing --rm -it --platform linux/amd64 --user root --hostname arceos \
    -v "$PWD:/workspace" "$image" "$@"
