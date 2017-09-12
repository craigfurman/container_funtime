#!/usr/bin/env bash
set -euo pipefail

project=$(cd "$(dirname "$0")"/.. && pwd)
docker run --privileged --rm -u 1000:1000 -v "$project":/tests -w /tests \
  rust:1.20-stretch cargo test --color always
