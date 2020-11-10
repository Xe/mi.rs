let
  sources = import ./nix/sources.nix;
  pkgs =
    import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
  rust = import ./nix/rust.nix { };
  gruvbox = pkgs.callPackage sources.gruvbox-css { };
in pkgs.mkShell rec {
  buildInputs = with pkgs; [
    # rust
    rust
    pkgconfig
    openssl
    cmake
    zlib
    diesel-cli
    sqlite
    libsodium
    cargo-watch

    # elm
    elmPackages.elm
    elmPackages.elm-format
    elmPackages.elm-language-server
    elm2nix

    # keep this line if you use bash
    bashInteractive
  ];

  GRUVBOX_CSS = "${gruvbox}/gruvbox.css";

  DATABASE_URL = "./mi.db";
  ROCKET_DATABASES = ''{ main_data = { url = "${DATABASE_URL}" } }'';
  RUST_LOG = "info";

  # libsodium-sys
  SODIUM_USE_PKG_CONFIG = "1";
  SODIUM_SHARED = "1";
}
