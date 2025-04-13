{
  description = "Build a cargo project without extra checks";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    crane,
    rust-overlay,
    ...
  }: let
    projectName = "zero2prod";
    system = "x86_64-linux";

    imageDetails = {
      inherit projectName;
      registry = "ghcr.io";
      owner = "edpft";
    };

    overlays = [(import rust-overlay)];
    pkgs = import nixpkgs {
      inherit system overlays;
      config = {
        allowUnfree = true;
      };
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

    # Check formatting
    fmt = craneLib.cargoFmt {
      inherit src;
    };

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
      name = "${imageDetails.registry}/${imageDetails.owner}/${imageDetails.projectName}";
      tag = "latest";
      copyToRoot = [bin];
      config = {
        Cmd = ["${bin}/bin/zero2prod"];
        ExposedPorts."8080/tcp" = {};
      };
    };

    moldDevShell = with pkgs;
      craneLib.devShell.override {
        mkShell = mkShell.override {
          stdenv = stdenvAdapters.useMoldLinker stdenv;
        };
      };
  in {
    checks.${system} = {
      inherit fmt clippy nextest;
    };

    packages.${system} = {
      inherit bin dockerImage;
      default = bin;
    };

    devShells.${system}.default = moldDevShell {
      inputsFrom = [bin];
      packages = with pkgs; [
        postman
        newman
      ];
    };
  };
}
