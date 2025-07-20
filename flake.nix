{
  description = "The Belalang Programming Language";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];

      imports = [ inputs.treefmt-nix.flakeModule ];

      perSystem =
        {
          pkgs,
          system,
          rust-toolchain,
          buildRustPackage,
          ...
        }:
        {
          _module.args = {
            pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [ inputs.rust-overlay.overlays.default ];
            };

            rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

            buildRustPackage =
              (pkgs.makeRustPlatform {
                cargo = rust-toolchain;
                rustc = rust-toolchain;
              }).buildRustPackage;
          };

          packages.default = buildRustPackage {
            name = "belalang";
            version = "0.1.0";
            src = ./.;

            cargoLock.lockFile = ./Cargo.lock;
          };

          devShells.default = pkgs.mkShell {
            name = "belalang";
            buildInputs = [
              rust-toolchain
              pkgs.cargo-nextest
            ];
          };

          treefmt.programs = {
            nixfmt.enable = true;
            rustfmt.enable = true;
            rustfmt.package = rust-toolchain;
          };
        };
    };
}
