let
  sources = import ./nix/sources.nix;
  pkgs =
    import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
  ruststable = (pkgs.latest.rustChannels.stable.rust.override { extensions = [ "rust-src" "rls-preview" "rust-analysis" "rustfmt-preview" ];});

in pkgs.mkShell {
  buildInputs = with pkgs; [
    ruststable
    pkgconfig
    openssl
    cmake
    zlib
    libgit2
    diesel-cli
    sqlite

    # keep this line if you use bash
    bashInteractive
  ];

  RUST_LOG="info";
  DATABASE_URL="./mi.db";
}
