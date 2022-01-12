# with import <nixpkgs> {};
# { stdenv, nixpkgs ? import <nixpkgs> {} }:
# { nixpkgs ? import <nixpkgs> }:
{ }:
let
  # moz_overlay = import (builtins.fetchTarball
  #   https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  # nightly_rust = (unstable.latest.rustChannels.nightly.rust.override {
  #   extensions = [];
  # });
  # unstable = import <nixos-unstable> { overlays = [ moz_overlay ]; };
  # mkRustCrate = callPackage ../lib/mkRustCrate {};
  unstable = import <nixos-unstable> {};
in
  unstable.rustPlatform.buildRustPackage rec {
    name = "xadjust";
    version = "0.1.0";
    src = ./.;
    buildInputs = [
      unstable.cargo
      unstable.rustc
    ];
    cargoSha256 = "1jwszhqmfr13n20jpij755yph6j9yb448pqy78xn6qwl5y3k5ssr";
  }
