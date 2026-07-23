{ pkgs, rustToolChain, ... }:

pkgs.stdenv.mkDerivation

{
  name = "rust journey";

  # Compile time dependencies
  nativeBuildInputs =

    with pkgs; [
      # Hail the Nix
      nixd
      statix
      deadnix
      alejandra

      # rust
      rustToolChain
      rustlings
    ];

  # Set Environment Variables
  RUST_BACKTRACE = "full";
  # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}