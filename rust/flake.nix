{
  description = "Pomodoro Timer - Rust/Relm4 Implementation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        
        buildInputs = with pkgs; [
          gtk4
          glib
          gobject-introspection
          pkg-config
        ];
        
        nativeBuildInputs = with pkgs; [
          rustc
          cargo
          rustfmt
          clippy
          pkg-config
          wrapGAppsHook4
        ];

        pomodoro-timer-rust = pkgs.rustPlatform.buildRustPackage {
          pname = "pomodoro-timer-rust";
          version = "0.1.0";
          
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          
          inherit buildInputs nativeBuildInputs;
          
          meta = with pkgs.lib; {
            description = "Pomodoro Timer built with Rust and Relm4";
            license = licenses.mit;
            platforms = platforms.linux;
            maintainers = [];
          };
        };

        # Development build script
        build-rust = pkgs.writeShellScriptBin "build-rust" ''
          set -e
          echo "🦀 Building Rust/Relm4 implementation..."
          cargo build
          echo "✅ Rust build complete! Run with: cargo run"
        '';
        
        # Run script
        run-rust = pkgs.writeShellScriptBin "run-rust" ''
          set -e
          echo "🚀 Starting Rust Pomodoro Timer..."
          cargo run
        '';

        # Test script
        test-rust = pkgs.writeShellScriptBin "test-rust" ''
          set -e
          echo "🧪 Running Rust tests..."
          cargo test
        '';

        # Example script
        example-rust = pkgs.writeShellScriptBin "example-rust" ''
          set -e
          echo "📝 Running timer usage example..."
          cargo run --example timer_usage
        '';

      in {
        packages = {
          default = pomodoro-timer-rust;
          pomodoro-timer-rust = pomodoro-timer-rust;
        };
        
        apps = {
          default = {
            type = "app";
            program = "${run-rust}/bin/run-rust";
          };
          build = {
            type = "app";
            program = "${build-rust}/bin/build-rust";
          };
          run = {
            type = "app";
            program = "${run-rust}/bin/run-rust";
          };
          test = {
            type = "app";
            program = "${test-rust}/bin/test-rust";
          };
          example = {
            type = "app";
            program = "${example-rust}/bin/example-rust";
          };
        };
        
        devShells.default = pkgs.mkShell {
          buildInputs = buildInputs ++ nativeBuildInputs ++ [
            build-rust
            run-rust
            test-rust
            example-rust
          ] ++ (with pkgs; [
            rust-analyzer
            gtk4.dev
          ]);
          
          shellHook = ''
            export GSK_RENDERER=gl
            export GTK_THEME=Adwaita:dark
            echo "🦀 Pomodoro Timer - Rust/Relm4 Implementation"
            echo "=============================================="
            echo ""
            echo "Available commands:"
            echo "  build-rust   - Build the Rust application"
            echo "  run-rust     - Run the Rust application"
            echo "  test-rust    - Run Rust tests"
            echo "  example-rust - Run timer usage example"
            echo "  nix run      - Run directly with nix"
            echo ""
            echo "Manual build:"
            echo "  cargo build  - Build with Cargo"
            echo "  cargo run    - Run with Cargo"
            echo "  cargo test   - Test with Cargo"
          '';
        };
      });
}