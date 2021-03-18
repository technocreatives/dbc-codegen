{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config

    nodejs-12_x
    python37 # for node-gyp
    can-utils
    clang
    llvmPackages.libclang.lib

    # keep this line if you use bash
    pkgs.bashInteractive
  ];

  # Needed so bindgen can find libclang.so
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}
