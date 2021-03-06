#!/bin/bash

# We'll destroy the release channel at the beginning of the pipeline,
# as well as at the end (whether we've succeeded or failed). This just
# ensures that we keep everything clean and tidy.

set -euo pipefail
source .buildkite/scripts/shared.sh

channel="$(get_release_channel)"
echo "--- Destroying release channel '${channel}'"

# TODO (CM): Once this command takes an --auth token, use that instead
HAB_AUTH_TOKEN="${HAB_AUTH_TOKEN}" hab bldr channel destroy \
    --origin=core \
    "${channel}"
