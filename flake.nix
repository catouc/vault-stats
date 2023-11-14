{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils/v1.0.0";
  };

  description = "";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "i686-linux" ] ( system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        build = pkgs.rustPlatform.buildRustPackage {
          pname = "vault-stats";
          version = "v0.1.0";
          src = ./.;
	  cargoSha256 = "sha256-TOGHkd2RJmxULWoXAYxj2xKts1VZGTUsu4FjjGU6Gyc=";
        };
      in
        rec {
        packages = {
          vault-stats = build;
          default = build;
        };

	hydraJobs = {inherit packages;};

        devShells = {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
	      cargo
	      rustc
	      rust-analyzer
	      vault
            ];
          };
        };
      }
    );
}

