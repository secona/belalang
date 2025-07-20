{
  description = "The Belalang Programming Language";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";

    git-hooks-nix.url = "github:cachix/git-hooks.nix";
    git-hooks-nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];

      imports = [
        inputs.git-hooks-nix.flakeModule
        inputs.treefmt-nix.flakeModule
      ];

      perSystem =
        {
          config,
          pkgs,
          system,
          rust-toolchain,
          buildRustPackage,
          ...
        }:
        let
          belalang = buildRustPackage {
            name = "belalang";
            version = "0.1.0";
            src = ./.;

            cargoLock.lockFile = ./Cargo.lock;
          };
        in
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

          packages.default = belalang;

          devShells.default = pkgs.mkShell {
            name = "belalang";
            buildInputs = [
              rust-toolchain
              pkgs.cargo-nextest
            ];
            shellHook = ''
              ${config.pre-commit.installationScript}
            '';
          };

          pre-commit = {
            check.enable = true;
            settings.hooks = {
              nixfmt-rfc-style.enable = true;
              rustfmt = {
                enable = true;
                packageOverrides.cargo = rust-toolchain;
                packageOverrides.rustfmt = rust-toolchain;
              };
            };
          };

          treefmt.programs = {
            nixfmt.enable = true;
            rustfmt.enable = true;
            rustfmt.package = rust-toolchain;
          };
        };
    };
}
