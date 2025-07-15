{
  description = "GNOME Pomodoro Timer with 20-20-20 Rule";

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

        pomodoro-timer = pkgs.stdenv.mkDerivation {
          pname = "pomodoro-timer";
          version = "1.0.0";
          
          src = ./.;
          
          inherit buildInputs nativeBuildInputs;
          
          mesonFlags = [
            "-Dprefix=${placeholder "out"}"
          ];
          
          postInstall = ''
            # Compile GSettings schemas
            glib-compile-schemas $out/share/glib-2.0/schemas
          '';
          
          meta = with pkgs.lib; {
            description = "GNOME Pomodoro Timer with 20-20-20 Rule";
            homepage = "https://github.com/user/pomodoro-timer";
            license = licenses.gpl3Plus;
            platforms = platforms.linux;
            maintainers = [];
          };
        };

        # Development build script
        build-dev = pkgs.writeShellScriptBin "build-dev" ''
          set -e
          echo "Building Pomodoro Timer..."
          
          # Setup build directory
          meson setup build --wipe 2>/dev/null || meson setup build
          
          # Build the application
          ninja -C build
          
          # Install schema locally for development
          mkdir -p ~/.local/share/glib-2.0/schemas
          cp data/com.github.user.PomodoroTimer.gschema.xml ~/.local/share/glib-2.0/schemas/
          glib-compile-schemas ~/.local/share/glib-2.0/schemas/
          
          echo "Build complete! Run with: ./build/src/pomodoro-timer"
        '';
        
        # Run script
        run-dev = pkgs.writeShellScriptBin "run-dev" ''
          set -e
          if [ ! -f "./build/src/pomodoro-timer" ]; then
            echo "Application not built. Building first..."
            build-dev
          fi
          
          echo "Starting Pomodoro Timer..."
          ./build/src/pomodoro-timer
        '';

      in {
        packages = {
          default = pomodoro-timer;
          pomodoro-timer = pomodoro-timer;
        };
        
        apps = {
          default = {
            type = "app";
            program = "${run-dev}/bin/run-dev";
          };
          build = {
            type = "app";
            program = "${build-dev}/bin/build-dev";
          };
          run = {
            type = "app";
            program = "${run-dev}/bin/run-dev";
          };
        };
        
        devShells.default = pkgs.mkShell {
          buildInputs = buildInputs ++ nativeBuildInputs ++ [
            build-dev
            run-dev
          ] ++ (with pkgs; [
            gdb
            valgrind
            gtk4.dev
            libadwaita.dev
          ]);
          
          shellHook = ''
            export GSK_RENDERER=gl
            export GTK_THEME=Adwaita:dark
            echo "🍅 GNOME Pomodoro Timer with 20-20-20 Rule"
            echo ""
            echo "Quick start:"
            echo "  build-dev    - Build the application"
            echo "  run-dev      - Build and run the application"
            echo "  nix run      - Run directly with nix"
            echo ""
            echo "Manual build:"
            echo "  meson setup build && ninja -C build"
            echo "  ./build/src/pomodoro-timer"
          '';
        };
      });
}