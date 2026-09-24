{
  pkgs,
  ...
}:

pkgs.stdenv.mkDerivation {
  pname = "xeonitte";
  version = "0.1.0";

  src = [ ./. ];

  cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
    src = ./.;
    hash = "";
  };

  nativeBuildInputs = with pkgs;[
    cargo
    pkg-config
    rustc
    rustPlatform.cargoSetupHook
  ];

  buildInputs = with pkgs; [
    rustPlatform.bindgenHook
  ];

  buildPhase = ''
    NIX_CFLAGS_COMPILE="$(pkg-config --cflags --libs MagickCore) $NIX_CFLAGS_COMPILE"
    # put the usual make/gcc code here
  '';
}
