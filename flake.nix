# SPDX-FileCopyrightText: 2022-2026 The Inuus Developers
#
# SPDX-License-Identifier: Apache-2.0

{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = inputs: let
    inherit (inputs) self nixpkgs flake-utils;
  in
    flake-utils.lib.eachDefaultSystem
    (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.inuus = pkgs.callPackage ./inuus.nix {};
      packages.default = self.packages.${system}.inuus;

      devShells.default = self.packages.${system}.default.overrideAttrs (super: {
        nativeBuildInputs = with pkgs;
          super.nativeBuildInputs
          ++ [
            cargo-edit
            clippy
            rustfmt
          ];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
      });
    })
    // {
      overlays.default = final: prev: {
        inherit (self.packages.${final.system}) inuus;
      };
    };
}
