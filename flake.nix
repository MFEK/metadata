{
  inputs = {
    nixpkgs.url      = github:nixos/nixpkgs/release-22.05;
    utils.url        = github:numtide/flake-utils;
    rust-overlay.url = github:oxalica/rust-overlay;
    naersk.url       = github:nix-community/naersk;


    # sample rust-skia build: https://github.com/NixOS/nixpkgs/blob/nixos-unstable/pkgs/applications/editors/neovim/neovide/default.nix#L114
    # rust-skia pulls skia from a hard-coded url by default (but flakes disallow internet access!),
    # so we have to ask it to look for system libraries then build skia locally
    skia = {
      url = github:rust-skia/skia;
      flake = false;
    };

    # Used for shell.nix
    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, utils, naersk, ... } @ inputs:
    let
      name = "MFEKmetadata";
      description = "Basic font metadata fetcher/updater for the MFEK project";
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
          nativeBuildInputs = with pkgs; [
            python3
          ];

          SKIA_USE_SYSTEM_LIBRARIES = true;
	  FORCE_SKIA_BUILD = true;
          # SKIA_LIBRARY_SEARCH_PATH
          # SKIA_SOURCE_DIR
        };
        # build error: "no such file or directory": /sources/skia-bindings-0.56.1/build_support/skia/config.rs:313:10
        # https://github.com/rust-skia/rust-skia/blob/8eb0dd7fc98dd21e5e8bd2fc866323399b3795f6/skia-bindings/build_support/skia/config.rs#L303
        # -- missing the skia dependency
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
