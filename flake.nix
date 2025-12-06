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
        rev = "2fc983e772acdcc40b4e8e75d276568b69a09792";
        hash = "sha256-D3BobeuHgaSfGijuuHTJItMHeO/AydBFv0QzLfQrIdo=";
      };

      cargoHash = "sha256-+A5JK0SAhLf1g4wwPNH5j2wqZWXHqU9kwE6ddtG0q7k=";

      meta = {
        description = "Blinkenlights for Syncthing";
        homepage = "https://github.com/make-42/lights-in-sync";
        license = pkgs.lib.licenses.mit;
        maintainers = with pkgs.lib.maintainers; [];
        mainProgram = "flow";
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
