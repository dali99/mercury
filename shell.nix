let
  pkgs = import <nixos-unstable> { };
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") { };
in
with pkgs;
{ windows-cross ? false }:
stdenv.mkDerivation {
  name = "rust_shell";

  nativeBuildInputs = []
    ++ lib.optionals windows-cross [
      pkgsCross.mingwW64.stdenv.cc
    ];

  buildInputs = [
    (fenix.combine ([
      fenix.complete.toolchain
    ] ++ lib.optionals windows-cross [
      fenix.targets.x86_64-pc-windows-gnu.latest.rust-std
    ]))

    (vscode-with-extensions.override {
      vscodeExtensions = with vscode-extensions; [
        bbenoist.nix
        vadimcn.vscode-lldb
        fenix.rust-analyzer-vscode-extension
      ];
    })

#    cargo-flamegraph

#    cmake
#    gcc
#    llvm_11
  ] ++ lib.optionals windows-cross [
    pkgsCross.mingwW64.windows.mingw_w64_pthreads
  ];
}
