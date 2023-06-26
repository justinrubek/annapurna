{
  description = "A rust project";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix-filter.url = "github:numtide/nix-filter";
    bomper = {
      url = "github:justinrubek/bomper";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    dream2nix = {
      url = "github:nix-community/dream2nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-parts.follows = "flake-parts";
    };
  };

  outputs = inputs @ {
    self,
    flake-utils,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux"];
      imports = [
        ./flake-parts/astro.nix
        inputs.dream2nix.flakeModuleBeta

        ./flake-parts/web.nix

        ./flake-parts/rust-toolchain.nix
        ./flake-parts/cargo.nix

        ./flake-parts/ci.nix
        ./flake-parts/shells.nix

        ./flake-parts/formatting.nix
        ./flake-parts/pre-commit.nix
        inputs.pre-commit-hooks.flakeModule
      ];
    };
}
