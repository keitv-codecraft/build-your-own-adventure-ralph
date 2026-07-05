{
  description = "A high-quality Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          # modern field (instead of buildInputs)
          packages = with pkgs; [
            rustToolchain
            pkg-config
            openssl

            # Task & Project Management
            go-task
            just

            # Testing & Coverage
            cargo-nextest
            cargo-llvm-cov

            # Code Quality & Auditing
            cargo-audit
            cargo-deny
            cargo-expand
            cargo-machete # Find unused dependencies
            cargo-update # Interactive dependency updates
            cargo-edit # Add/remove deps from CLI

            # Development Tools
            cargo-watch # File watching for development
            bacon # Background rust code checker

            # Linker (mold for faster linking)
            mold # Modern linker for faster builds

            # Utilities
            xdg-utils
          ];

          # optional but cleaner than manual export
          env = {
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            # Use mold as the linker for faster builds
            RUSTFLAGS = "-C link-arg=-fuse-ld=mold";
          };
        };
      }
    );
}
