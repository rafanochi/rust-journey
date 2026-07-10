{
  description = "A simple Rust dev shell using Nix Flakes";

  inputs = {
    # Use the unstable channel for the latest packages
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # flake-utils to support multiple systems easily
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustfmt
            clippy
            rustc
            rust-analyzer
          ];
        }
    );

}

