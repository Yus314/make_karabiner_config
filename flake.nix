{
  description = "Rust env";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      fenix,
      flake-utils,
      nixpkgs,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        #toolchain = fenix.packages.${system}.fromToolchainFile {
        #  file = ./rust-toolchain.toml;
        #  sha256 = "sha256-AJ6LX/Q/Er9kS15bn9iflkUwcgYqRQxiOIL2ToVAXaU=";
        #};
        overlays = [ (import rust-overlay) ];
        #pkgs = nixpkgs.legacyPackages.${system};
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        formatter = pkgs.nixpkgs-fmt;
        devShells.default = pkgs.stdenv.mkDerivation {
          name = "rust environment";
          nativeBuildInputs = [
            #   toolchain
            rust-toolchain
          ];
          buildInputs = with pkgs; [
            #  # rust-toolchain
            #  cargo
            #  rustc
            #  rust-analyzer
            #  rustfmt
            #  clippy
            #];
            #nativeBuildInputs = with pkgs; [
            #  rustc
            #  cargo
            #  gcc
            cowsay
          ];
          #shellHook = ''
          #              	    export RUST_SRC_PATH="${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}"
          ### 	    export CARGO="${pkgs.cargo}"
          #            	  '';
        };
      }
    );
}
