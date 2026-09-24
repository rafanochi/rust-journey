{ pkgs, ... }:

let
  rustVersion = "latest";
  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    extensions = [
      "rustc"
      "cargo"
      "rustfmt"
      "clippy"
      "rust-analyzer"
      "rust-src"
      # "cargo-watch"
    ];
  };
in

pkgs.stdenv.mkDerivation {
  name = "relm4";

  # Compile time dependencies
  nativeBuildInputs = with pkgs; [
    # Hail the Nix
    nixd
    statix
    deadnix
    alejandra

    # Other compile time dependencies
    openssl
    # libressl

    rust
    cargo-watch

    # pkg-config
    rustPlatform.bindgenHook

    # image magick
    imagemagick
    libjpeg
    libpng
    pkg-config
  ];

  # Set Environment Variables
  RUST_BACKTRACE = "full";
  MAGICKCORE_HDRI_ENABLE = "true";
  # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
