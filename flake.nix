{
  description = "Local-First Zero - Agentic Office Suite with CRDT sync";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Build inputs for the project
        # NOTE: Using rustls instead of openssl - see global feedback memory
        buildInputs = with pkgs; [
          pkg-config
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        # Native build inputs
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
        ];

      in
      {
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          packages = with pkgs; [
            # Rust tooling
            cargo-watch
            cargo-edit
            cargo-nextest

            # Code quality
            rustfmt
            clippy

            # Documentation
            mdbook

            # WASM tooling (add wac, wit-deps when available in nixpkgs)
            wasm-tools
          ];

          shellHook = ''
            echo "Office Suite Dev Environment"
            echo "Rust: $(rustc --version)"
            echo ""
            echo "Commands:"
            echo "  cargo check        - Check compilation"
            echo "  cargo test         - Run tests"
            echo "  cargo watch -x test - Auto-run tests on change"
          '';
        };

        packages.default = pkgs.rustPlatform.buildCargoPackage {
          pname = "office-suite";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          inherit buildInputs nativeBuildInputs;
        };
      }
    );
}
