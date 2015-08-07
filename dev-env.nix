let
  pkgs    = import <nixpkgs> {};
  stdenv  = pkgs.stdenv;
  lib     = pkgs.lib;

in rec {
  devEnv = stdenv.mkDerivation rec {
    name = "nickel-sqlite-dev-env";
    src = ./.;
    buildInputs = with pkgs; [
      pkgconfig
      rustPlatform.rustc
      cargo
      sqlite
      openssl
    ];
  };
}
