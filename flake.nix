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
      version = "0-unstable-2025-12-06-0";

      src = pkgs.fetchFromGitHub {
        owner = "make-42";
        repo = "lights-in-sync";
        rev = "425e5c07e0a1b4e4fe5f777c358c5149891a41b6";
        hash = "sha256-A1h/NPmQ25q5CaV5CYemCpuiciD2KBacRwKcQMfpE84=";
      };

      cargoHash = "sha256-bwdwGLuNlzKKXs2nc0ApGXtDZMD0U9ny+QQGQ4ybSOM=";

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
