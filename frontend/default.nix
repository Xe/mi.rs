{ sources ? import ../nix/sources.nix
, pkgs ? import sources.nixpkgs {}
}:

with pkgs;

let
  mkDerivation =
    { srcs ? ./elm-srcs.nix
    , src
    , name
    , srcdir ? "./src"
    , targets ? []
    , registryDat ? ./registry.dat
    , outputJavaScript ? false
    }:
    stdenv.mkDerivation {
      inherit name src;

      buildInputs = [ elmPackages.elm ];

      buildPhase = pkgs.elmPackages.fetchElmDeps {
        elmPackages = import srcs;
        elmVersion = "0.19.1";
        inherit registryDat;
      };

      installPhase = let
        elmfile = module: "${srcdir}/${builtins.replaceStrings ["."] ["/"] module}.elm";
        extension = if outputJavaScript then "js" else "html";
      in ''
        mkdir -p $out/share/doc
        ${lib.concatStrings (map (module: ''
          echo "compiling ${elmfile module}"
          elm make ${elmfile module} --output $out/${module}.${extension} --docs $out/share/doc/${module}.json
        '') targets)}
      '';
    };
in mkDerivation {
  name = "mi-frontend-0.1.0";
  srcs = ./elm-srcs.nix;
  src = ./.;
  targets = ["Main"];
  srcdir = "./src";
  outputJavaScript = true;
}

