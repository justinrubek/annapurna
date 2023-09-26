{inputs, ...}: {
  perSystem = {
    config,
    lib,
    pkgs,
    system,
    inputs',
    self',
    ...
  }: let
    # wasm-bindgen modules to include in the files
    wasm-modules = {
      loader = inputs'.wasm-bindgen-service-worker.packages.loader;
      ui = self'.packages.ui;
      wasm = self'.packages.wasm;
      service-worker = self'.packages.service-worker;
    };

    # generate bash script that copies the contents of all the wasm-modules into their respective directories
    wasm-modules-includes = {
      # attrset of modules to include
      modules,
      # directory name inside public to copy the modules into
      dirName,
    }:
      lib.concatMapStringsSep "\n" (name: ''
        mkdir -p $out/public/${dirName}/${name}
        cp -r ${modules.${name}}/* $out/public/${dirName}/${name}
      '') (builtins.attrNames modules);

    static-files = pkgs.runCommand "static-files" {} ''
      mkdir -p $out

      mkdir -p $out/public/wasm

      cp -r ${../public}/* $out/public

      ${wasm-modules-includes {
        modules = wasm-modules;
        dirName = "wasm";
      }}

      # the service worker needs to be in the root of the public directory in order to have scope over the entire site
      cp ${self'.packages.service-worker}/* $out/public
    '';
  in rec {
    packages = {
      inherit static-files;
      serve = pkgs.writeShellApplication {
        name = "serve-annapurna";
        runtimeInputs = [pkgs.miniserve];
        text = ''
          miniserve ${static-files}/public "$@"
        '';
      };
    };

    apps = {
      serve = {
        type = "app";
        program = packages.serve;
      };
    };
  };
}
