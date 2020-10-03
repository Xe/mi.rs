let
  sources = import ./nix/sources.nix;
  pkgs =
    import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
  ruststable = (pkgs.latest.rustChannels.stable.rust.override {
    extensions = [ "rust-src" "rls-preview" "rust-analysis" "rustfmt-preview" ];
  });

in pkgs.mkShell {
  buildInputs = with pkgs; [
    # rust
    ruststable
    pkgconfig
    openssl
    cmake
    zlib
    libgit2
    diesel-cli
    sqlite

    # elm
    elmPackages.elm
    elmPackages.elm-format
    elmPackages.elm-language-server
    elm2nix

    # keep this line if you use bash
    bashInteractive
  ];

  DATABASE_URL = "./mi.db";
  RUST_LOG = "info";
}
