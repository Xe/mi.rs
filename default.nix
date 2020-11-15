{ sources ? import ./nix/sources.nix, pkgs ? import sources.nixpkgs { } }:
with pkgs;
let
  backend = pkgs.callPackage ./backend { inherit sources pkgs; };
  frontend = pkgs.callPackage ./sina { inherit sources pkgs; };
  gruvbox = pkgs.callPackage sources.gruvbox-css { };
in stdenv.mkDerivation {
  pname = "mi";
  version = "${backend.version}";
  phases = "installPhase";

  installPhase = ''
    # service binary
    mkdir -p $out/bin

    for file in ${backend}/bin/*; do
      ln -s ${backend}/bin/$(${coreutils}/bin/basename $file) $out/bin/$(${coreutils}/bin/basename $file)
    done

    # frontend JS
    mkdir -p $out/public/js
    ln -s ${frontend}/Main.js $out/public/js/elm.js

    # static files
    cp -vrf ${./static}/* $out/public

    # migrations
    ln -s ${./backend/migrations} $out/migrations
  '';
}
