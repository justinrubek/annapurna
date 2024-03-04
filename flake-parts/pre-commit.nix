{
  inputs,
  self,
  ...
}: {
  perSystem = {self', ...}: let
  in {
    pre-commit = {
      check.enable = false;

      settings = {
        src = ../.;
        hooks = {
          treefmt.enable = true;

          sql-prepare = {
            enable = true;
            entry = "cargo sqlx prepare --workspace";
            # add `--check` to check only. Without it the file will be updated when the hook is run
            # entry = "cargo sqlx prepare --workspace --check";
            pass_filenames = false;
          };
        };

        settings.treefmt.package = self'.packages.treefmt;
      };
    };
  };
}
