{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";

    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs, nixpkgs-mozilla }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;

          overlays = [
            (import nixpkgs-mozilla)
          ];
        };

        toolchain = (pkgs.rustChannelOf {
          date = "2023-08-03";
          channel = "stable";
          sha256 = "sha256-R0F0Risbr74xg9mEYydyebx/z0Wu6HI0/KWwrV30vZo=";
        }).rust.override {
            targets = [
                "aarch64-apple-darwin"
                "aarch64-pc-windows-msvc"
                "aarch64-unknown-linux-musl"
                "armv7-unknown-linux-gnueabihf"
                "x86_64-apple-darwin"
                "x86_64-pc-windows-msvc"
                "x86_64-unknown-freebsd"
                "x86_64-unknown-linux-musl"
            ];
            extensions = [
                "rust-src"
                "rust-analyzer-preview"
                "rust-std"
                "clippy-preview"
                "rustfmt-preview"
                "llvm-tools-preview"
            ];
        };

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

      in
      rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ ];
          cargoBuildOptions = (opts: opts ++ [ "--all-features" "--all-targets" ]);
        };

        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell {
          nativeBuildInputs = [ toolchain ] ++ (with pkgs; [ pkg-config ]);
          buildInputs = with pkgs; [ ];
        };
      }
    );
}
