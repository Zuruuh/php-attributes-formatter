{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/adbc678ea2850981c363f0531c710baa191e269d";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
      };

      toolchain =
        (fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-vMlz0zHduoXtrlu0Kj1jEp71tYFXyymACW8L4jzrzNA=";
        });

      nativeBuildInputs = with pkgs; [
        toolchain
        pkg-config
      ];

      buildInputs = with pkgs;[ libiconvReal ];
    in
    {
      devShell = pkgs.mkShell {
        inherit buildInputs nativeBuildInputs;
      };
    });
}
