{
  description = "Chessy flake for reproducible builds!";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    forAllSystems = {
      pkgs ? nixpkgs,
      function,
    }:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "x86_64-macos"
        "aarch64-linux"
        "aarch64-darwin"
      ]
      (system:
        function {
          pkgs = import pkgs {
            inherit system;
            config.allowUnfree = true;
            overlays = [
              rust-overlay.overlays.default
            ];
          };
          inherit system;
        });
  in {
    devShells = forAllSystems {
      function = {pkgs, ...}: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            # Backend and Engine
            (rust-bin.fromRustupToolchainFile ./rust-toolchain)
            openssl
            pkg-config
            wasm-pack
            websocat

            # VueJS Frontend
            nodejs
            yarn

            # Elm Frontend
            elmPackages.elm
            elmPackages.elm-format
          ];

          shellHook = ''
            export RUST_BACKTRACE=1
          '';
        };
      };
    };
  };
}
