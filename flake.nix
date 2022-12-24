let
  name = "MFEKmetadata";
  description = "Basic font metadata fetcher/updater for the MFEK project";
in {
  inherit name description;

  inputs = {
    nixpkgs.url      = github:nixos/nixpkgs/release-22.05;
    utils.url        = github:numtide/flake-utils;
    rust-overlay.url = github:oxalica/rust-overlay;
    naersk.url       = github:nix-community/naersk;

    # Used for shell.nix
    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, utils, naersk, ... } @ inputs: let
      overlays = [ rust-overlay.overlays.default ];
      # Our supported systems are the same supported systems as the Rust binaries
      systems = builtins.attrNames inputs.rust-overlay.packages;
    in utils.lib.eachSystem systems (system:
      let
        pkgs = import nixpkgs { inherit overlays system; };
        rust_channel = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust_channel;
          rustc = rust_channel;
        };
      in {
        defaultPackage = naersk-lib.buildPackage {
          pname = name;
          root = ./.;
        };

        devShells.default = pkgs.mkShell {
          inherit name description;
          buildInputs = with pkgs; [
            rust_channel
            rust-analyzer
            cargo
            lld
            pkg-config
            fontconfig
            freetype
          ];
          # for rust-analyzer; the target dir of the compiler for the project
          OUT_DIR = "./target";
        };

        # For compatibility with older versions of the `nix` binary
        devShell = self.devShells.${system}.default;
      });
}
