{
  description = "A rust project";

  inputs = {
    awatch = {
      url = "github:justinrubek/async-watcher";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.fenix.follows = "fenix";
    };
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
    wasm-bindgen = {
      url = "github:justinrubek/wasm-bindgen";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.fenix.follows = "fenix";
    };
    wasm-bindgen-service-worker = {
      url = "github:justinrubek/wasm-bindgen-service-worker";
    };
    nix2container = {
      url = "github:nlewo/nix2container";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    process-compose.url = "github:Platonic-Systems/process-compose-flake";
    services-flake.url = "github:justinrubek/services-flake";
    nix-postgres.url = "github:justinrubek/nix-postgres";
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
        ./flake-parts/web.nix

        ./flake-parts/rust-toolchain.nix
        ./flake-parts/cargo.nix

        ./flake-parts/ci.nix
        ./flake-parts/shells.nix

        ./flake-parts/formatting.nix
        ./flake-parts/pre-commit.nix
        inputs.pre-commit-hooks.flakeModule

        ./flake-parts/containers.nix

        ./flake-parts/postgres.nix
        ./flake-parts/sqlx.nix

        inputs.process-compose.flakeModule
        ./flake-parts/services.nix
      ];
    };
}
