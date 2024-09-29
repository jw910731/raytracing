{
  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs = { nixpkgs.follows = "nixpkgs"; };
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, systems, fenix, ... } @ inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = forEachSystem (system: {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
      });

      devShells = forEachSystem
        (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
            lib = pkgs.lib;
          in
          {
            default = devenv.lib.mkShell {
              inherit inputs pkgs;
              modules = [
                ({config, ...}: let 
                  channel = "stable";
                  toolchain = fenix.packages.${pkgs.stdenv.system}.${channel}.toolchain;
                  cargo-instruments =  (pkgs.makeRustPlatform {
                    cargo = toolchain;
                    rustc = toolchain;
                  }).buildRustPackage rec {
                    pname = "cargo-instruments";
                    version = "0.4.10";

                    buildInputs = with pkgs; [
                      sccache libgit2 pkg-config libiconv llvmPackages_13.libclang openssl
                      darwin.apple_sdk.frameworks.SystemConfiguration
                      darwin.apple_sdk.frameworks.CoreServices
                    ];

                    src = pkgs.fetchFromGitHub {
                      owner = "cmyr";
                      repo = pname;
                      rev = "v${version}";
                      hash = "sha256-dtCjZxYvCEmACeUBHJ3g8pJmKXI6YnKvbw6GxiPIPWE=";
                    };

                    cargoHash = "sha256-R82svOcGv1xhqHYFDY9sqeP5nE9SbpyZJAj6eZB+M+k=";
                  };
                in{
                  # https://devenv.sh/reference/options/
                  packages = [  ] ++ lib.optionals pkgs.stdenv.isDarwin [
                    cargo-instruments
                  ];
                  languages.rust = {
                    inherit toolchain channel;
                    enable = true;
                  };
                })
              ];
            };
          });
    };
}
