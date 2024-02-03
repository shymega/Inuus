# SPDX-FileCopyrightText: 2023 The Steam-ToyBox Developers
#
# SPDX-License-Identifier: GPL-3.0-or-later

{ lib
, pkgs ? import <nixpkgs>
, rustPlatform
,
}:
rustPlatform.buildRustPackage {
  name = "steam-toybox";

  src = lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    # Allow dependencies to be fetched from git and avoid having to set the outputHashes manually
    allowBuiltinFetchGit = true;
  };

  nativeBuildInputs = with pkgs; [ pkg-config cmake openssl.dev ];
  buildInputs = with pkgs; [ openssl.dev ];

  meta = with lib; {
    description = "";
    homepage = "https://github.com/shymega/Steam-ToyBox";
    license = licenses.mit;
  };
}
