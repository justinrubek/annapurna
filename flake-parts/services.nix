{inputs, ...} @ part-inputs: {
  imports = [];

  perSystem = {
    pkgs,
    inputs',
    self',
    ...
  }: let
    ports = {
      annapurna-postgres = "\${ANNAPURNA_POSTGRES_PORT}";
    };

    global-imports = [
      ({name, ...}: {
        dataDirEnv = "\${PRJ_DATA_HOME}/${name}";
        socketDirEnv = "\${PRJ_DATA_HOME}/${name}/sockets";
      })
    ];
  in rec {
    process-compose = {
      services = {
        imports = [
          inputs.services-flake.processComposeModules.default
        ];

        services.postgres."annapurna-postgres" = {
          imports = global-imports;

          enable = true;
          package = self'.packages.postgresql;

          # by not including a listen address, we only listen on the unix socket
          # listen_addresses = "127.0.0.1";
          port = ports.annapurna-postgres;
          initialDatabases = [
            {
              name = "annapurna-local";
            }
          ];
        };
      };
    };
  };
}
