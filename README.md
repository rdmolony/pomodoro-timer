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

### Method 1: NixOS System Package

Add to your NixOS configuration:

```nix
# configuration.nix or flake.nix
{
  environment.systemPackages = with pkgs; [
    (pkgs.callPackage (builtins.fetchGit {
      url = "https://github.com/user/pomodoro-timer";
      rev = "main";  # or specific commit
    }) {})
  ];
}
```

Then rebuild: `sudo nixos-rebuild switch`

### Method 2: User Installation

```bash
# Clone and install to user profile
git clone https://github.com/user/pomodoro-timer
cd pomodoro-timer
nix build
nix profile install ./result

# The app will be available in your applications menu
```

### Method 3: Temporary Installation

```bash
# Install temporarily (removed on reboot)
git clone https://github.com/user/pomodoro-timer
cd pomodoro-timer
nix shell

# Or install from GitHub directly
nix shell github:user/pomodoro-timer
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

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes following the code style
4. Test thoroughly: `nix develop && run-dev`
5. Submit a pull request

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