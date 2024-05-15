{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            nodePackages.vscode-langservers-extracted
            nodePackages.typescript-language-server
            tailwindcss-language-server
            tailwindcss
            openssl
            pkg-config
            cacert
            cargo-watch
            trunk
            taplo
            wasm-pack
            chromedriver
            bashInteractive
            (rust-bin.stable.latest.default.override {
                extensions= [ "rust-src" "rust-analyzer" ];
                targets = [ "wasm32-unknown-unknown" ];
            })
          ];
          shellHook = ''
          '';
        };
      }
    );
}
