{
  inputs,
  self,
  ...
} @ part-inputs: {
  imports = [];

  perSystem = {
    config,
    pkgs,
    lib,
    system,
    inputs',
    self',
    ...
  }: let
    # packages required for building the rust packages
    extraPackages = [
      pkgs.pkg-config
    ];
    withExtraPackages = base: base ++ extraPackages;

    craneLib = (inputs.crane.mkLib pkgs).overrideToolchain self'.packages.rust-toolchain;

    commonArgs = rec {
      src = inputs.nix-filter.lib {
        root = ../.;
        include = [
          "crates"
          "Cargo.toml"
          "Cargo.lock"
          ".sqlx"
        ];
      };

      pname = "annapurna";

      nativeBuildInputs = withExtraPackages [];
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
      SQLX_OFFLINE = true;
    };

    cargoArtifacts = craneLib.buildDepsOnly (commonArgs
      // {
        cargoExtraArgs = "--exclude annapurna-wasm --exclude annapurna-ui --workspace";
      });
    packages = let
      buildWasmPackage = {
        name,
        wasm-bindgen-target ? "web",
      }: let
        underscore_name = pkgs.lib.strings.replaceStrings ["-"] ["_"] name;

        # It isn't possible to build the full workspace for wasm, so it is duplicated here
        wasmArgs =
          commonArgs
          // {
            pname = "${commonArgs.pname}-deps-wasm";
            cargoExtraArgs = "--package ${name}";
            CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
            doCheck = false;
          };

        cargoArtifactsWasm = craneLib.buildDepsOnly (
          wasmArgs
          // {
          }
        );

        cargo-derivation = craneLib.buildPackage ({
            cargoArtifacts = cargoArtifactsWasm;
          }
          // wasmArgs);

        wasm-derivation = pkgs.stdenv.mkDerivation {
          name = "${name}-wasm";
          buildInputs = [inputs'.wasm-bindgen.packages.wasm-bindgen-cli];
          nativeBuildInputs = [pkgs.binaryen];
          src = "";
          buildCommand = ''
            ${inputs'.wasm-bindgen.packages.wasm-bindgen-cli}/bin/wasm-bindgen \
              ${cargo-derivation}/lib/${underscore_name}.wasm \
              --out-dir $out \
              --target ${wasm-bindgen-target} \

            ${pkgs.binaryen}/bin/wasm-opt \
              -Oz \
              --output $out/${underscore_name}_bg.wasm \
              $out/${underscore_name}_bg.wasm
          '';
        };
      in
        wasm-derivation;
    in {
      default = packages.cli;
      cli = craneLib.buildPackage ({
          pname = "annapurna-cli";
          inherit cargoArtifacts;
          cargoExtraArgs = "--bin annapurna-cli";
          meta.mainProgram = "annapurna-cli";
        }
        // commonArgs);

      service-worker = buildWasmPackage {
        name = "annapurna-service-worker";
      };

      ui = buildWasmPackage {
        name = "annapurna-ui";
      };

      wasm = buildWasmPackage {
        name = "annapurna-wasm";
      };

      cargo-doc = craneLib.cargoDoc ({
          inherit cargoArtifacts;
        }
        // commonArgs);
    };

    checks = {
      clippy = craneLib.cargoClippy (commonArgs
        // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-features -- --deny warnings";
        });

      rust-fmt = craneLib.cargoFmt (commonArgs
        // {
          inherit (commonArgs) src;
        });

      rust-tests = craneLib.cargoNextest (commonArgs
        // {
          inherit cargoArtifacts;
          partitions = 1;
          partitionType = "count";
          cargoNextestExtraArgs = "--exclude annapurna-ui --workspace";
        });
    };
  in rec {
    inherit packages checks;

    apps = {
      cli = {
        type = "app";
        program = pkgs.lib.getBin self'.packages.cli;
      };
      default = apps.cli;
    };

    legacyPackages = {
      cargoExtraPackages = extraPackages;
    };
  };
}
