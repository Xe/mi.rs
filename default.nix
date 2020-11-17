{ sources ? import ./nix/sources.nix, pkgs ? import sources.nixpkgs { } }:
with pkgs;
let
  backend = callPackage ./backend { inherit sources pkgs; };
  frontend = callPackage ./sina { inherit sources pkgs; };
  gruvbox = callPackage sources.gruvbox-css { };

  composite = stdenv.mkDerivation {
    pname = "mi";
    version = "${backend.version}";
    phases = "installPhase";

    installPhase = ''
      # service binary
      mkdir -p $out/bin

      for file in ${backend}/bin/*; do
        ln -s ${backend}/bin/$(basename $file) $out/bin/$(basename $file)
      done

      # static files
      mkdir -p $out/public/
      cp -vrf ${./static}/* $out/public

      # frontend JS
      rm $out/public/elm.js
      ln -s ${frontend}/Main.min.js $out/public/elm.js
    '';
  };

  wrapper = writeScriptBin "mi-backend" ''
    #!${pkgs.stdenv.shell}
    set -e
    set -x

    export RUST_LOG=info
    export DATABASE_URL=./mi.db
    export ROCKET_DATABASES='{ main_data = { url = "./mi.db" } }';
    ${composite}/bin/migrate_database
    export ROCKET_ASSET_PATH=${composite}/public
    exec ${composite}/bin/mi
  '';
in symlinkJoin {
  name = "mi";
  paths = [ wrapper composite ];
}
