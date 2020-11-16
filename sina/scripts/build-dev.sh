#!/usr/bin/env nix-shell
#! nix-shell -i bash -p elmPackages.elm

echo "--------------- rebuilding ------------------"
elm make ./src/Main.elm --output elm.js
echo "------------------ done ---------------------"
