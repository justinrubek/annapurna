{inputs, ...}: {
  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    self',
    lib,
    ...
  }: let
    inherit (self'.packages) rust-toolchain postgresql sqlx-cli sqlx-migrations;
    inherit (self'.legacyPackages) cargoExtraPackages ciPackages;

    devTools = [
      # rust tooling
      rust-toolchain
      pkgs.cargo-audit
      pkgs.cargo-udeps
      pkgs.cargo-nextest
      pkgs.bacon
      inputs'.wasm-bindgen.packages.wasm-bindgen-cli
      # formatting
      self'.packages.treefmt
      # database
      postgresql
      sqlx-cli
      sqlx-migrations
    ];
  in {
    devShells = {
      default = pkgs.mkShell rec {
        packages = devTools ++ cargoExtraPackages ++ ciPackages;

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
        RUST_SRC_PATH = "${self'.packages.rust-toolchain}/lib/rustlib/src/rust/src";

        shellHook = ''
          ${config.pre-commit.installationScript}
        '';
      };
    };
  };
}
