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
    ,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        fenixPkgs = fenix.packages.${system};

        host-toolchain = fenixPkgs.toolchainOf {
          channel = "nightly";
          date = "2026-03-21";
          sha256 = "sha256-rboGKQLH4eDuiY01SINOqmXUFUNr9F4awoFZGzib17o=";
        };
        wasm-toolchain = (fenixPkgs.targets.wasm32-unknown-unknown.toolchainOf {
          channel = "nightly";
          date = "2026-03-21";
          sha256 = "sha256-rboGKQLH4eDuiY01SINOqmXUFUNr9F4awoFZGzib17o=";
        }).rust-std;

        toolchain = fenixPkgs.combine [
          (host-toolchain.withComponents [
            "cargo"
            "clippy"
            "rustc"
            "rustfmt"
            "rust-analyzer"
            "rust-src"
            "llvm-tools-preview"
            "rustc-codegen-cranelift-preview"
          ])
          wasm-toolchain
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          name = "rust";

          nativeBuildInputs = [
            toolchain
            pkgs.mold
            pkgs.bacon

            pkgs.nodejs_24
            pkgs.typescript-language-server
            pkgs.svelte-language-server
            pkgs.tailwindcss-language-server
          ];

          buildInputs = with pkgs; [
            SDL2
          ];

          env.RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";

          shellHook = ''
            echo -e "\033[0;32mDone!\033[0m"
          '';
        };
      }
    );
}
