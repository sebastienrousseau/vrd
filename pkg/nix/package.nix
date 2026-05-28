# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Package definition for the `vrd` CLI binary. Used by the
# flake at ../flake.nix and as the source for a future
# nixpkgs PR.

{ lib
, rustPlatform
, fetchFromGitHub
}:

rustPlatform.buildRustPackage rec {
  pname = "vrd";
  version = "__VERSION__";

  src = fetchFromGitHub {
    owner = "sebastienrousseau";
    repo = "vrd";
    rev = "v${version}";
    sha256 = "__SHA256__";
  };

  cargoHash = "sha256-__CARGO_HASH__=";

  meta = with lib; {
    description = "Lightweight, no_std-friendly random number generator for Rust";
    homepage = "https://vrdlib.com";
    license = with licenses; [ asl20 mit ];
    maintainers = with maintainers; [ ];   # add yourself on PR
    platforms = platforms.unix ++ platforms.windows;
    mainProgram = "vrd";
  };
}
