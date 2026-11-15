{
  description = "Graphina: A graph data science library for Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nixpkgs-python310.url = "github:NixOS/nixpkgs/nixos-25.05";
    lean4-nix = {
      url = "github:lenianiva/lean4-nix/2ad74592e34c2e6ff5e8bec9d8abab8f4e9c3a37";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    aeneas.url = "github:AeneasVerif/aeneas/1dc8ad31b1b062a77c50e29144a6c7bba955fa09";
  };

  outputs = { self, nixpkgs, nixpkgs-python310, lean4-nix, aeneas }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import aeneas.inputs.charon.inputs.rust-overlay) ];
          };
          leanPkgs = import nixpkgs {
            inherit system;
            overlays = [ (lean4-nix.readToolchainFile ./verification/lean-toolchain) ];
          };
          # lean4-nix has no manifest for this release candidate yet.
          extractionLean = (pkgs.callPackage "${lean4-nix}/lib/toolchain.nix" { }).fetchBinaryLean {
            tag = "v4.36.0-rc2";
            rev = "b35cb10cf2ced51a2fd1880cfdbf75c71ab7a35a";
            inherit (import "${lean4-nix}/manifests/v4.35.0.nix") bootstrap;
            toolchain = {
              x86_64-linux.hash = "sha256:0a0a3208617e9eab92f1041efa83ad94d405e69d172fb654a62dbfa9fbc5313b";
              aarch64-linux.hash = "sha256:7ac84c80ddd19dfbe4277bc50ee916fca0fec36500d30dadaae9c5e904ec618d";
              aarch64-darwin.hash = "sha256:800c09adb81ac667fd9e8e4211d54bdc5cc5e193b107599f6ae72f69ee9b91ad";
            };
          };
          # The main package set predates stable Python 3.15.
          python315 = aeneas.inputs.nixpkgs.legacyPackages.${system}.python315;
          developmentShell = python: pkgs.mkShell {
            UV_PYTHON = "${python}/bin/python3";
            LD_LIBRARY_PATH = pkgs.lib.optionalString pkgs.stdenv.isLinux
              (pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc.lib pkgs.zlib ]);
            # Keep Nix applications' Python libraries out of uv environments.
            shellHook = ''unset PYTHONPATH'';
            packages = with pkgs; [
              (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
              git
              pkg-config
              cargo-nextest
              cargo-tarpaulin
              cargo-audit
              cargo-deny
              cargo-careful
              python
              uv
              maturin
              pre-commit
            ];
          };
        in
        {
          default = developmentShell python315;
          verification = pkgs.mkShell {
            packages = [ leanPkgs.lean pkgs.git pkgs.gnumake pkgs.curl ];
          };
          publishing = pkgs.mkShell {
            packages = [ pkgs.uv pkgs.python3Packages.twine ];
          };
          extraction = pkgs.mkShell {
            # Charon's sysroot differs from Rustup's even at the same Rust version.
            CARGO_TARGET_DIR = "target/charon";
            packages = [
              aeneas.packages.${system}.aeneas
              aeneas.inputs.charon.packages.${system}.rustToolchain
              extractionLean
              pkgs.git
              pkgs.gnumake
              pkgs.pkg-config
            ];
          };
        } // pkgs.lib.genAttrs
          [ "python310" "python311" "python312" "python313" "python314" "python315" ]
          (name: developmentShell (
            if name == "python310" then nixpkgs-python310.legacyPackages.${system}.python310
            else if name == "python315" then python315
            else pkgs.${name}
          ))
      );

      checks = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          verification-tools = pkgs.runCommand "graphina-verification-tools" {
            nativeBuildInputs = self.devShells.${system}.verification.nativeBuildInputs;
          } ''
            lean --version | grep -F 'version 4.35.0'
            lake --version
            echo 'example : (1 + 1 : Nat) = 2 := by decide' > Smoke.lean
            lean Smoke.lean
            touch $out
          '';
          extraction-tools = pkgs.runCommand "graphina-extraction-tools" {
            nativeBuildInputs = self.devShells.${system}.extraction.nativeBuildInputs;
          } ''
            charon version | grep -F '454586e9b770231e66d2062993e632dfc06463e2'
            aeneas -version | grep -F '1dc8ad3'
            rustc --version | grep -F '1.100.0-nightly'
            lean --version | grep -F 'version 4.36.0-rc2'
            echo 'pub fn identity(x: bool) -> bool { x }' > smoke.rs
            charon rustc --preset=aeneas --dest-file smoke.llbc -- --crate-type lib smoke.rs
            mkdir generated
            aeneas -backend lean smoke.llbc -dest generated -namespace Smoke -warnings-as-errors -no-progress-bar
            test -s generated/Smoke.lean
            touch $out
          '';
        } // pkgs.lib.optionalAttrs pkgs.stdenv.isLinux {
          python-wheel-runtime = pkgs.runCommand "graphina-python-wheel-runtime" {
            inherit (self.devShells.${system}.python310) LD_LIBRARY_PATH;
          } ''
            ${self.devShells.${system}.python310.UV_PYTHON} -c 'import ctypes; ctypes.CDLL("libz.so.1"); ctypes.CDLL("libstdc++.so.6")'
            touch $out
          '';
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
