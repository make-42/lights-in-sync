{
  description = "lights-in-sync - A topbar program for getting and displaying Syncthing folder statuses.";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
    };
  in {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage rec {
      pname = "lights-in-sync";
      version = "0-unstable-2025-12-06";

      src = pkgs.fetchFromGitHub {
        owner = "make-42";
        repo = "lights-in-sync";
        rev = "71bf691b23107b548f794caad1cd5e6e487d49be";
        hash = "sha256-EYJDp1en7fRg6o21b7dmp4QPaWWssy6JUd8WJVaNJh8=";
      };

      cargoHash = "sha256-+A5JK0SAhLf1g4wwPNH5j2wqZWXHqU9kwE6ddtG0q7k=";

      meta = {
        description = "Blinkenlights for Syncthing";
        homepage = "https://github.com/make-42/lights-in-sync";
        license = pkgs.lib.licenses.mit;
        maintainers = with pkgs.lib.maintainers; [];
        mainProgram = "lights-in-sync";
      };
    };

    devShells.default = pkgs.mkShell {
      name = "lights-in-sync-devshell";

      buildInputs = with pkgs; [
        # Rust toolchain
        rustc
        cargo
        cargo-watch
        cargo-audit
        rustfmt
        clippy

        # Helpful extras
        llvmPackages.bintools
      ];

      RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
    };

    defaultPackage.${system} = self.packages.${system}.default;
  };
}
