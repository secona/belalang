{
  description = "The Belalang Programming Language";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };

      belalang = pkgs.rustPlatform.buildRustPackage {
        pname = "belalang";
        version = "0.1.0";
        cargoLock.lockFile = ./Cargo.lock;
        src = pkgs.lib.cleanSource ./.;
      };
    in {
      packages.default = belalang;

      devShells.default = pkgs.mkShell {
        name = "belalang";

        inputsFrom = [belalang];

        buildInputs = with pkgs; [
          rust-analyzer
          rustfmt
          clippy
        ];
      };
    });
}
