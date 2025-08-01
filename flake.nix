{
  description = "rust development flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        toolchainFile = ./rust-toolchain.toml;
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile toolchainFile;
      in
      {
        devShells = {
          default = pkgs.mkShell {
            buildInputs = [
              rustToolchain
            ];
          };
        };
      }
    );
}
