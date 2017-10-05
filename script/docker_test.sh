#!/bin/bash
set -euo pipefail

project=$(cd "$(dirname "$0")"/.. && pwd)
docker run --privileged --rm -u dave:dave -v "$project":/tests -w /tests \
  craigfurman/container-funtime-testing /tests/script/test_in_container.sh
