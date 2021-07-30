{ sources ? import ./sources.nix }:

let
  pkgs =
    import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
  chan = pkgs.rustChannelOfTargets "nightly" "2021-07-29" [];
in chan
