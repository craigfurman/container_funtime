#!/bin/bash
set -euo pipefail

(
cd "$(dirname "$0")"/../integration_tests
bundle install
bundle exec rspec
)
