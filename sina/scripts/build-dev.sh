#!/usr/bin/env nix-shell
#! nix-shell -i bash -p elmPackages.elm

elm make ./src/Main.elm --output elm.js
