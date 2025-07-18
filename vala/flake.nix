{
  description = "Pomodoro Timer - Vala/GTK4 Implementation";

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
          libadwaita
          glib
          gobject-introspection
          pkg-config
          wrapGAppsHook4
          desktop-file-utils
          appstream-glib
          libnotify
          gsound
        ];
        
        nativeBuildInputs = with pkgs; [
          meson
          ninja
          pkg-config
          vala
          gobject-introspection
          wrapGAppsHook4
          desktop-file-utils
          appstream-glib
        ];

        pomodoro-timer-vala = pkgs.stdenv.mkDerivation {
          pname = "pomodoro-timer-vala";
          version = "1.0.0";
          
          src = ./.;
          
          inherit buildInputs nativeBuildInputs;
          
          mesonFlags = [];
          
          postInstall = ''
            # Compile GSettings schemas
            glib-compile-schemas $out/share/glib-2.0/schemas
          '';
          
          meta = with pkgs.lib; {
            description = "Pomodoro Timer built with Vala and GTK4";
            license = licenses.gpl3Plus;
            platforms = platforms.linux;
            maintainers = [];
          };
        };

        # Development build script
        build-vala = pkgs.writeShellScriptBin "build-vala" ''
          set -e
          echo "🍅 Building Vala/GTK4 implementation..."
          
          # Setup build directory
          meson setup build --wipe 2>/dev/null || meson setup build
          
          # Build the application
          ninja -C build
          
          # Install schema locally for development
          mkdir -p ~/.local/share/glib-2.0/schemas
          cp data/com.github.user.PomodoroTimer.gschema.xml ~/.local/share/glib-2.0/schemas/
          glib-compile-schemas ~/.local/share/glib-2.0/schemas/
          
          echo "✅ Vala build complete! Run with: ./build/src/pomodoro-timer"
        '';
        
        # Run script
        run-vala = pkgs.writeShellScriptBin "run-vala" ''
          set -e
          if [ ! -f "./build/src/pomodoro-timer" ]; then
            echo "Application not built. Building first..."
            build-vala
          fi
          
          echo "🚀 Starting Vala Pomodoro Timer..."
          ./build/src/pomodoro-timer
        '';

        # Test script
        test-vala = pkgs.writeShellScriptBin "test-vala" ''
          set -e
          echo "🧪 Running Vala tests..."
          if [ ! -d "build" ]; then
            echo "Building first..."
            build-vala
          fi
          meson test -C build
        '';

      in {
        packages = {
          default = pomodoro-timer-vala;
          pomodoro-timer-vala = pomodoro-timer-vala;
        };
        
        apps = {
          default = {
            type = "app";
            program = "${run-vala}/bin/run-vala";
          };
          build = {
            type = "app";
            program = "${build-vala}/bin/build-vala";
          };
          run = {
            type = "app";
            program = "${run-vala}/bin/run-vala";
          };
          test = {
            type = "app";
            program = "${test-vala}/bin/test-vala";
          };
        };
        
        devShells.default = pkgs.mkShell {
          buildInputs = buildInputs ++ nativeBuildInputs ++ [
            build-vala
            run-vala
            test-vala
          ] ++ (with pkgs; [
            gdb
            valgrind
            gtk4.dev
            libadwaita.dev
          ]);
          
          shellHook = ''
            export GSK_RENDERER=gl
            export GTK_THEME=Adwaita:dark
            echo "🍅 Pomodoro Timer - Vala/GTK4 Implementation"
            echo "=============================================="
            echo ""
            echo "Available commands:"
            echo "  build-vala   - Build the Vala application"
            echo "  run-vala     - Build and run the Vala application"
            echo "  test-vala    - Run Vala tests"
            echo "  nix run      - Run directly with nix"
            echo ""
            echo "Manual build:"
            echo "  meson setup build && ninja -C build"
            echo "  ./build/src/pomodoro-timer"
          '';
        };
      });
}