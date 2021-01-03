let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixos-unstable> { overlays = [ moz_overlay ]; };
  rustNightlyChannel = (nixpkgs.rustChannelOf { date = "2020-03-19"; channel = "nightly"; }).rust.override {
    extensions = [
			"rust-src"
			"rls-preview"
			"clippy-preview"
			"rustfmt-preview"
		];
  };
	rustStableChannel = nixpkgs.latest.rustChannels.stable.rust.override {
		extensions = [
			"rust-src"
			"rls-preview"
			"clippy-preview"
			"rustfmt-preview"
		];
  };
in
with nixpkgs;
  stdenv.mkDerivation {
    name = "moz_overlay_shell";
    buildInputs = [
      rustStableChannel
      (vscode-with-extensions.override {
        vscodeExtensions = with vscode-extensions; [
          bbenoist.Nix
          vadimcn.vscode-lldb
          matklad.rust-analyzer
        ];
      })
    ];
  }
