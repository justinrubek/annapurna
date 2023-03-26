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
    inherit (inputs.nix-filter.lib) filter inDirectory matchExt;
  in rec {
    packages = {
      inherit (pkgs) nodejs yarn;

      inherit (config.dream2nix.outputs.web.packages) web;
    };

    dream2nix.inputs.web = {
      source = filter {
        root = ../web;
        include = [
          "yarn.lock"
          (inDirectory "src")
          (inDirectory "public")
          (matchExt "js")
          (matchExt "json")
          (matchExt "ts")
          (matchExt "cjs")
          (matchExt "mjs")
        ];
      };

      projects.web = {
        subsystem = "nodejs";
        translator = "yarn-lock";
      };

      packageOverrides.web = {
        # Instead of packaging the library, only package the final build
        copySite = {
          installPhase = ''
            mkdir -p $out
            cp -r ./dist/* $out
          '';
        };
      };
    };
  };
}
