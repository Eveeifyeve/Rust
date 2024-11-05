{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devenv.flakeModule ];
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem =
        {
          config,
          self',
          inputs',
          lib,
          pkgs,
          ...
        }:
        let 
        inherit (pkgs.darwin.apple_sdk.frameworks) Security SystemConfiguration;
        in
        {
          devenv.shells.default = {
                        dotenv.enable = false;
            packages = lib.optionals pkgs.stdenv.isDarwin
              [
                Security
                SystemConfiguration
              ] ++ [ pkgs.cargo-leptos];
            languages.rust = {
              enable = true;
              channel = "stable";
              toolchain = {
                rustc = pkgs.rustc-wasm32;
              };
              targets = [ "wasm32-unknown-unknown" ];
            };
          };
        };
    };
}
