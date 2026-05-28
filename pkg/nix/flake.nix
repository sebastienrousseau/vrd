# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Flake exposing the `vrd` CLI binary. Reachable via:
#   nix run github:sebastienrousseau/vrd
# without any further setup on a NixOS or nix-the-package-
# manager-on-anywhere install.

{
  description = "Versatile Random Distributions (vrd) - Rust RNG library + CLI demo";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.callPackage ./package.nix { };
        packages.vrd = self.packages.${system}.default;

        # `nix develop` drops you into a shell with the build deps.
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
          ];
        };
      });
}
