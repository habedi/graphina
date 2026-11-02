{
  description = "Graphina: A graph data science library for Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
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
              rustup
              pkg-config
              cargo-nextest
              cargo-tarpaulin
              cargo-audit
              cargo-careful
              python315
              uv
              maturin
              python3Packages.mkdocs-material
              pre-commit
            ];
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
          };
        }
      );
    };
}
