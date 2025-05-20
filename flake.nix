{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crate2nix = {
      url = "github:nix-community/crate2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nix2container = {
      url = "github:nlewo/nix2container";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      crate2nix,
      nix2container,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        meta = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
        inherit (meta) name version;

        pkgs = import nixpkgs {
          inherit system;

          overlays = [
            rust-overlay.overlays.default
          ];
        };
        nix2containerPkgs = nix2container.packages.${system};

        generatedCargoNix = crate2nix.tools.${system}.generatedCargoNix {
          inherit name;
          src = ./.;
        };

        cargoNix = import generatedCargoNix {
          inherit pkgs;

          buildRustCrateForPkgs =
            crate:
            pkgs.buildRustCrate.override {
              rustc = pkgs.rust-bin.stable.latest.default;
              cargo = pkgs.rust-bin.stable.latest.default;
            };
        };
      in
      rec {
          apps.copyDockerImage = {
            type = "app";
            program = builtins.toString (pkgs.writeShellScript "copyDockerImage" ''
              IFS=$'\n' # iterate over newlines
              set -x # echo on
              for DOCKER_TAG in $DOCKER_METADATA_OUTPUT_TAGS; do
                ${pkgs.lib.getExe self.packages.${system}.dockerImage.copyTo} "docker://$DOCKER_TAG"
              done
            '');
          };

        packages = rec {
          multipathd-exporter = cargoNix.workspaceMembers.multipathd-exporter.build;
          default = packages.multipathd-exporter;

          dockerImage = nix2containerPkgs.nix2container.buildImage {
            name = "ghcr.io/vexxhost/multipathd-exporter";
            tag = version;
            maxLayers = 64;
            copyToRoot = with pkgs.dockerTools; [
              pkgs.multipath-tools
            ];

            config = {
              entrypoint = [ "${default}/bin/multipathd-exporter" ];
              ExposedPorts = {
                "10035/tcp" = { };
              };
            };
          };
        };
      }
    );
}
