{
  description = "Flake for Rust Development";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , fenix
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        fenixLib = fenix.packages.${system};

        rustToolchain = fenixLib.latest.toolchain;
      in
      {
        devShells.default = pkgs.mkShell {
          name = "rust";

          nativeBuildInputs = [
            rustToolchain

            pkgs.clang
            pkgs.mold
          ];

          # buildInputs = with pkgs; [
          #   SDL2
          # ];

          shellHook = ''
            echo -e "\033[0;32mDone!\033[0m"
          '';
        };
      }
    );
}
