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

pkgs.mkShell {
  name = "relm4";

  # Compile time dependencies
  packages = with pkgs; [
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


    pkg-config
    git
    direnv
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    cargo-dist
    cargo-release
    cargo-expand
    imagemagick
    llvmPackages.libclang
  ];

  # Set Environment Variables
  RUST_BACKTRACE = "full";
  # MAGICKCORE_HDRI_ENABLE = "true";
  # LD_LIBRARY_PATH = "${pkgs.imagemagick}/lib64";
  # CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true;
  # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  env = {
    LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
    # Required env var for the `build.rs` of magick_rust to use tho correct ImageMagick CLANG flags.
    # This variable is used here: https://github.com/nlfiedler/magick-rust/blob/dfd8df0dd102348c23b33bfc946a9d70b5db25bf/build.rs#L127C9-L127C28
    # Not setting this variable will throw the error: "you should set MAGICKCORE_HDRI_ENABLE"
    "BINDGEN_EXTRA_CLANG_ARGS_${pkgs.stdenv.hostPlatform.rust.rustcTarget}" = "-DMAGICKCORE_HDRI_ENABLE=1 -DMAGICKCORE_QUANTUM_DEPTH=16 -DMAGICKCORE_CHANNEL_MASK_DEPTH=32 -I${pkgs.imagemagick.dev}/include/ImageMagick-7";
  };

  shellHook = ''
    export BINDGEN_EXTRA_CLANG_ARGS="$BINDGEN_EXTRA_CLANG_ARGS -DMAGICKCORE_HDRI_ENABLE=1 -DMAGICKCORE_QUANTUM_DEPTH=16 -DMAGICKCORE_CHANNEL_MASK_DEPTH=32 -I${pkgs.imagemagick.dev}/include/ImageMagick-7";
  '';
}
