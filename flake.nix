{
  description = "The Belalang Programming Language";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      naersk,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs { inherit system overlays; };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
          clippy = toolchain;
        };

        belalang = naersk'.buildPackage {
          name = "belalang";
          version = "0.1.0";
          src = ./.;
        };
      in
      {
        formatter = pkgs.nixfmt-rfc-style;

        packages.default = belalang;

        devShells.default = pkgs.mkShell {
          name = "belalang";
          buildInputs = [ toolchain pkgs.cargo-nextest ];
        };
      }
    );
}
