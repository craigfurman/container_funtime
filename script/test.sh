#!/bin/bash
set -euo pipefail

(
cd "$(dirname "$0")"/..
cargo build
cd integration_tests
bundle install
bundle exec rspec
)
