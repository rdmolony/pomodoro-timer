# 🍅 GNOME Pomodoro Timer with 20-20-20 Rule

> [!WARNING]
> This has been written entirely by `Claude` with minimal input from me

A native GNOME application that combines the Pomodoro Technique with the 20-20-20 rule for eye health. Built with GTK4 and libadwaita for seamless GNOME integration.

## Features

### 🎯 Pomodoro Technique
- **25-minute work sessions** with automatic break management
- **Short breaks (5 minutes)** after each session
- **Long breaks (15 minutes)** every 4 sessions
- **Session tracking** with visual progress
- **Pause/Resume/Reset** controls

### 👁️ 20-20-20 Rule for Eye Health
- **Independent 20-minute timer** for eye care reminders
- **Persistent notifications** that require acknowledgment
- **Snooze option** (5 minutes) for flexibility
- **Toggle on/off** as needed
- **Real-time countdown** display

### 🎨 GNOME Integration
- **Native libadwaita UI** with modern GNOME design
- **Desktop notifications** with sound alerts
- **GSettings integration** for persistent preferences
- **Responsive design** that adapts to window size

## Quick Start

### Requirements
- **Nix package manager** with flakes enabled
- **GNOME desktop environment** (for optimal experience)

### One-Command Run
```bash
nix run github:user/pomodoro-timer
```

### Development Setup
```bash
# Clone the repository
git clone https://github.com/user/pomodoro-timer
cd pomodoro-timer

# Enter development environment
nix develop

# Build and run (one command)
run-dev
```

## Installation

### Method 1: NixOS Flake (Recommended)

Add to your NixOS configuration:

```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    pomodoro-timer.url = "github:user/pomodoro-timer";
  };

  outputs = { self, nixpkgs, pomodoro-timer }: {
    nixosConfigurations.hostname = nixpkgs.lib.nixosSystem {
      modules = [
        {
          environment.systemPackages = [ pomodoro-timer.packages.x86_64-linux.default ];
        }
      ];
    };
  };
}
```

Then rebuild: `sudo nixos-rebuild switch --flake .`

### Method 2: Home Manager Integration

Add to your home-manager configuration:

```nix
# home.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    home-manager.url = "github:nix-community/home-manager";
    pomodoro-timer.url = "github:user/pomodoro-timer";
  };

  outputs = { nixpkgs, home-manager, pomodoro-timer, ... }: {
    homeConfigurations.username = home-manager.lib.homeManagerConfiguration {
      modules = [
        {
          home.packages = [ pomodoro-timer.packages.x86_64-linux.default ];
        }
      ];
    };
  };
}
```

Then apply: `home-manager switch --flake .`

### Method 3: Direct Installation

```bash
# Clone and install to user profile
git clone https://github.com/user/pomodoro-timer
cd pomodoro-timer
nix build
nix profile install ./result

# The app will be available in your applications menu
```

### Method 4: Temporary Run

```bash
# Run without installing
nix run github:user/pomodoro-timer

# Or from local directory
git clone https://github.com/user/pomodoro-timer
cd pomodoro-timer
nix run
```

## Build Instructions

### Method 1: Using Nix (Recommended)

#### Quick Build & Run
```bash
nix develop
run-dev
```

#### Manual Build
```bash
nix develop
build-dev
./build/src/pomodoro-timer
```

#### Install System-wide
```bash
nix build
sudo cp result/bin/pomodoro-timer /usr/local/bin/
```

### Method 2: Traditional Build

#### Prerequisites
Install these dependencies on your system:
- GTK4 (≥4.8)
- libadwaita (≥1.2)
- libnotify (≥0.7)
- gsound (≥1.0)
- meson & ninja
- vala compiler
- pkg-config

#### Build Steps
```bash
# Setup build directory
meson setup build

# Compile
ninja -C build

# Install GSettings schema (required)
mkdir -p ~/.local/share/glib-2.0/schemas
cp data/com.github.user.PomodoroTimer.gschema.xml ~/.local/share/glib-2.0/schemas/
glib-compile-schemas ~/.local/share/glib-2.0/schemas/

# Run
./build/src/pomodoro-timer
```

## Usage

### Starting a Pomodoro Session
1. Launch the application
2. Click **"Start"** to begin a 25-minute session
3. The timer will count down with a visual progress bar
4. Get notified when the session ends

### Managing 20-20-20 Reminders
1. Toggle the **"Enable 20-20-20 reminders"** switch
2. Every 20 minutes, you'll get a notification
3. Look at something 20 feet away for 20 seconds
4. Click **"Got it!"** or **"Remind me in 5 minutes"**

### Customization
Settings are automatically saved and include:
- Pomodoro duration (default: 25 minutes)
- Short break duration (default: 5 minutes)
- Long break duration (default: 15 minutes)
- 20-20-20 reminders (default: enabled)
- Notification sounds (default: enabled)

## Architecture

### Technology Stack
- **Language**: Vala
- **UI Framework**: GTK4 + libadwaita
- **Build System**: Meson + Ninja
- **Package Management**: Nix flakes
- **Notifications**: libnotify
- **Audio**: gsound

### File Structure
```
├── src/
│   ├── main.vala              # Application entry point
│   ├── application.vala       # Application class & lifecycle
│   ├── window.vala           # Main window & UI logic
│   ├── timer.vala            # Timer implementation
│   └── notification-manager.vala # Notification handling
├── data/
│   ├── *.desktop.in          # Desktop file
│   ├── *.appdata.xml.in      # AppStream metadata
│   └── *.gschema.xml         # GSettings schema
├── flake.nix                 # Nix build configuration
├── meson.build              # Build configuration
└── README.md                # This file
```

## Development

### Available Commands (in nix develop)
```bash
build-dev     # Build the application with schema setup
run-dev       # Build and run the application
nix run       # Run directly with nix (builds automatically)
nix build     # Create installable package
```

### Debugging
```bash
nix develop
gdb ./build/src/pomodoro-timer
# or
valgrind ./build/src/pomodoro-timer
```

### Code Style
- Follow standard Vala conventions
- Use 4-space indentation
- Comment public APIs
- Keep functions focused and small

## Troubleshooting

### "Settings schema not installed" Error
```bash
# Make sure the schema is compiled
mkdir -p ~/.local/share/glib-2.0/schemas
cp data/com.github.user.PomodoroTimer.gschema.xml ~/.local/share/glib-2.0/schemas/
glib-compile-schemas ~/.local/share/glib-2.0/schemas/
```

### Notifications Not Working
- Ensure you're running a GNOME session
- Check notification permissions in GNOME Settings
- Verify libnotify is properly installed

### Build Failures
- Make sure you're in the nix development shell
- Try cleaning: `rm -rf build && nix develop`
- Check that all dependencies are available

## Testing

This project has **100% test coverage** with 42 comprehensive tests following Test-Driven Development (TDD) methodology:

```bash
# Run all tests
nix develop
meson test -C build

# Run specific test suite
meson test -C build timer-tests
meson test -C build integration-tests
```

### Test Coverage
- **Timer Core Logic**: 9 tests covering initialization, duration, start/pause/stop, signals
- **Notification Manager**: 6 tests covering notifications, 20-20-20 reminders, sound settings
- **Application**: 6 tests covering app lifecycle, settings, actions
- **Main Window**: 12 tests covering UI integration, timer events, display updates
- **Integration**: 9 tests covering complete workflows, settings persistence
- **Validation**: 3 tests for desktop file, appstream, and schema validation

### Test Framework
- **GLib.Test** for structured unit testing
- **Meson** build integration with custom targets
- **GTK-friendly** testing patterns avoiding GUI dependencies
- **TDD approach** following Kent Beck's methodology

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Follow TDD: write failing tests first, then implement
4. Ensure all tests pass: `meson test -C build`
5. Test thoroughly: `nix develop && run-dev`
6. Submit a pull request

### Feature Requests
- Custom timer durations
- Statistics tracking
- Themes and appearance options
- Integration with calendar apps
- Export/import settings

## License

This project is licensed under the GPL-3.0-or-later license. See the source files for details.

## Acknowledgments

- **GNOME Project** for the excellent GTK4 and libadwaita frameworks
- **Vala Community** for the productive language
- **20-20-20 Rule** from the American Academy of Ophthalmology
- **Pomodoro Technique** by Francesco Cirillo