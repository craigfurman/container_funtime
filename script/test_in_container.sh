#!/bin/bash
set -euo pipefail

export GEM_HOME=$(ruby -e 'print Gem.user_dir')
export PATH="$(ruby -e 'print Gem.user_dir')/bin:$PATH"
"$(dirname "$0")"/test.sh
