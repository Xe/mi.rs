#!/usr/bin/env nix-shell
#! nix-shell -i bash -p entr

find ./src/* | entr ./scripts/build-dev.sh
