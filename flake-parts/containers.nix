{inputs, ...}: {
  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    self',
    ...
  }: let
    nix2container = inputs'.nix2container.packages.nix2container;
  in rec {
    packages."annapurna/docker" = nix2container.buildImage {
      name = "annapurna";
      config = {
        entrypoint = ["${self'.packages.cli}/bin/annapurna-cli"];
      };
      copyToRoot = pkgs.buildEnv {
        name = "root";
        paths = [
          self'.packages.static-files
          self'.packages.facts
          self'.packages.sqlx-cli
          self'.packages.sqlx-migrations
          pkgs.cacert
        ];
        pathsToLink = [
          "/bin"
          "/etc"
          "/facts"
          "/migrations"
          "/public"
        ];
      };
    };
  };
}
