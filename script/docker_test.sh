#!/bin/bash
set -euo pipefail

project=$(cd "$(dirname "$0")"/.. && pwd)
docker run --privileged --rm -u dave:dave -w /tests \
  -v "$project":/tests -v "$CTR_TESTS_ALPINE_ROOTFS":/rootfs/alpine \
  -e CTR_TESTS_ALPINE_ROOTFS=/rootfs/alpine \
  craigfurman/container-funtime-testing /tests/script/test_in_container.sh
