{
  description = "Graphina: A graph data science library for Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              # Rust toolchain (rustup honors the pinned rust-toolchain.toml)
              rustup

              # Build dependencies
              pkg-config

              # Visualization backend (plotters renders via fontconfig and freetype)
              fontconfig
              freetype

              # Rust development tools
              cargo-nextest
              cargo-tarpaulin
              cargo-audit
              cargo-careful

              # Python bindings (PyGraphina) toolchain
              python3
              uv
              maturin

              # Documentation
              python3Packages.mkdocs-material

              # Git hooks
              pre-commit
            ];

            # Ensure the linker can find fontconfig and freetype for the visualization feature
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
              fontconfig
              freetype
            ]);
          };
        }
      );

      packages = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "graphina";
            version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            # Build the library crate only; PyGraphina is built separately with maturin.
            cargoBuildFlags = [ "--package" "graphina" "--features" "all" ];
            cargoTestFlags = [ "--package" "graphina" "--features" "all" ];

            nativeBuildInputs = with pkgs; [
              pkg-config
            ];

            buildInputs = with pkgs; [
              fontconfig
              freetype
            ];
          };
        }
      );
    };
}
