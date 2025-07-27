{
  # run with `nix develop`
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
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

        rust-wasm = pkgs.rust-bin.stable.latest.default.override {
          targets = ["wasm32-unknown-unknown"];
          extensions = ["rust-src" "llvm-tools-preview"];
        };

      in {
        devShells.default = pkgs.mkShell {
          packages = [
            rust-wasm
            pkgs.trunk
            pkgs.wasm-bindgen-cli
            pkgs.binaryen
            pkgs.leptosfmt
            pkgs.tailwindcss
          ];

          RUSTFLAGS = "-C target-feature=+crt-static";

          shellHook = ''
            trunk serve
           '';
        };
      });
}