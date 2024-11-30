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
          rustup
          (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          udev
          alsa-lib
          vulkan-loader
          libxkbcommon
          wayland
        ];
      in
      with pkgs;
      {
        formatter = nixpkgs.legacyPackages.${system}.nixfmt-rfc-style;
        devShells.default = mkShell {
          nativeBuildInputs = [
            pkg-config
          ];
          inherit buildInputs;
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      }
    );
}
