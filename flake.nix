{
  description = "Build a cargo project without extra checks";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];

      pkgs = import nixpkgs {
        inherit system overlays;
      };

      # Use the rust version specified in our `rust-toolchain.toml`
      rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      src = craneLib.cleanCargoSource ./.;
      nativeBuildInputs = [rustToolchain];

      commonArgs = {
        inherit src nativeBuildInputs;
        strictDeps = true;
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      cargoClippyExtraArgs = "--all-targets -- --warn clippy::pedantic --deny warnings";
      clippy = craneLib.cargoClippy (commonArgs
        // {
          inherit cargoArtifacts cargoClippyExtraArgs;
        });

      nextest = craneLib.cargoNextest (commonArgs
        // {
          inherit cargoArtifacts;
          partitions = 1;
          partitionType = "count";
          cargoNextestPartitionsExtraArgs = "--no-tests=pass";
        });

      bin = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
        });

      dockerImage = pkgs.dockerTools.buildImage {
        name = "zero2prod";
        tag = "latest";
        copyToRoot = [bin];
        config = {
          Cmd = ["${bin}/bin/zero2prod"];
        };
      };

      moldDevShell = with pkgs;
        craneLib.devShell.override {
          mkShell = mkShell.override {
            stdenv = stdenvAdapters.useMoldLinker stdenv;
          };
        };
    in {
      checks = {
        inherit bin clippy nextest;
      };

      packages = {
        inherit bin dockerImage;
        default = bin;
      };

      devShells.default = moldDevShell {
        inputsFrom = [bin];
      };
    });
}
