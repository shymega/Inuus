{
  lib,
  pkgs,
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  name = "inuus";

  src = lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    # Allow dependencies to be fetched from git and avoid having to set the outputHashes manually
    allowBuiltinFetchGit = true;
  };

  nativeBuildInputs = with pkgs; [pkg-config cmake openssl.dev];
  buildInputs = with pkgs; [openssl.dev];

  meta = with lib; {
    description = "";
    homepage = "https://github.com/shymega/Inuus";
    license = licenses.mit;
  };
}
