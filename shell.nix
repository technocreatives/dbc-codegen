{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config

    nodejs-12_x
    python37 # for node-gyp
    can-utils

    # keep this line if you use bash
    pkgs.bashInteractive
  ];
}
