{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
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
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        buildInputs = with pkgs; [
          (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libxcb
          libxkbcommon
          alsa-lib
          libudev-zero
          openssl
          llvm
          pkg-config
          gcc
          sqlite
        ];
      in
      with pkgs;
      {
        formatter = nixpkgs.legacyPackages.${system}.nixfmt-rfc-style;
        devShells.default = mkShell {
          nativeBuildInputs = [
            pkg-config
          ];
          # inherit buildInputs;
          buildInputs = buildInputs ++ [ pkgs.valgrind ];
          LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${
            pkgs.lib.makeLibraryPath [
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              libxkbcommon
              xorg.libxcb
              pkgs.vulkan-loader
              pkgs.glfw
            ]
          }";
        };
      }
    );
}
