# Pomodoro Timer - Rust/Relm4 Implementation

A minimal pomodoro timer application built with Rust and the Relm4 framework for GTK4.

## Architecture

This is a **Relm4-based** implementation that demonstrates functional reactive programming patterns in Rust:

- **Model-View-Update (MVU)** pattern with reactive UI updates
- **Message-passing** architecture for handling user interactions
- **Minimal state management** with automatic UI synchronization

## Features

- ✅ 25-minute pomodoro timer
- ✅ Start/Pause/Reset functionality
- ✅ Real-time countdown display
- ✅ Clean, minimal GTK4 interface

## Structure

```
src/
├── main.rs           # Application entry point
├── main_app.rs       # Main Relm4 component
├── app_model.rs      # Application state management
├── timer.rs          # Core timer logic
├── timer_model.rs    # Timer UI components
└── lib.rs            # Library exports and tests
```

## Dependencies

- **relm4** - Reactive GTK4 framework
- **gtk4** - GUI toolkit

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Implementation Notes

### Relm4 Architecture
- Uses `SimpleComponent` trait for the main application
- Implements reactive UI updates through the `update_view` method
- Handles user input through message passing (`AppMsg::Timer`)

### Timer Logic
- Core timer functionality is separate from UI concerns
- Uses GLib's timeout mechanism for periodic updates
- Implements proper state management for start/pause/reset

### Code Quality
- Follows Rust best practices and idioms
- Minimal dependencies (only 2 crates)
- Comprehensive test coverage for core functionality
- Clean separation of concerns

---

This implementation serves as a comparison point for evaluating different approaches to building the same application in various languages and frameworks.