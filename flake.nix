{
  description = "Rust flake template";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, fenix }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            fenix.overlay
          ];
        };
      in
      {


        devShell = with pkgs; mkShell {
          shellHook = ''
          '';
          buildInputs = with pkgs; [
            (pkgs.fenix.complete.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
              "rust-analyzer"
            ])
          ];
        };
      });
}
