# Pomodoro Timer: Vala vs Rust Implementation Comparison

This project demonstrates two different approaches to building the same GTK4 application using different languages and frameworks.

## Architecture Comparison

### Vala Implementation (`vala/`)
- **Language**: Vala (compiles to C)
- **Framework**: GTK4 with GObject
- **Architecture**: Object-oriented with GObject signals
- **Build System**: Meson
- **Features**: Full-featured with settings, notifications, eye-check reminders

### Rust Implementation (`rust/`)
- **Language**: Rust
- **Framework**: Relm4 (reactive GTK4 framework)
- **Architecture**: Functional reactive programming (Model-View-Update)
- **Build System**: Cargo
- **Features**: Minimal core functionality (timer only)

## Key Differences

| Aspect | Vala | Rust |
|--------|------|------|
| **Paradigm** | Object-oriented | Functional/Reactive |
| **Memory Management** | Automatic (GC) | Manual (ownership) |
| **Message Passing** | GObject signals | Relm4 messages |
| **State Management** | Mutable objects | Immutable updates |
| **UI Updates** | Imperative | Declarative |
| **Dependencies** | GTK4, GLib | Relm4, GTK4 |
| **Lines of Code** | ~800 lines | ~300 lines |

## Architectural Patterns

### Vala: Object-Oriented with Signals
```vala
// Traditional GTK approach
button.clicked.connect(() => {
    timer.start();
    update_ui();
});
```

### Rust: Reactive with Messages
```rust
// Relm4 reactive approach
start_button.connect_clicked(move |_| {
    sender.input(AppMsg::Timer(TimerMsg::Start));
});
```

## Performance Characteristics

- **Vala**: Compiles to optimized C code, minimal runtime overhead
- **Rust**: Zero-cost abstractions, excellent performance with memory safety

## Development Experience

### Vala Advantages
- ✅ Familiar to C# and Java developers
- ✅ Direct GTK4 API access
- ✅ Mature ecosystem
- ✅ Excellent IDE support

### Rust Advantages
- ✅ Memory safety without garbage collection
- ✅ Excellent type system
- ✅ Fearless concurrency
- ✅ Growing ecosystem

## Conclusion

Both implementations achieve the same core functionality but demonstrate different approaches:

- **Vala** provides a more traditional, imperative approach similar to other GTK applications
- **Rust/Relm4** offers a modern, reactive approach with strong type safety and memory management

The choice between them depends on team expertise, performance requirements, and architectural preferences.

## Running the Implementations

### Vala
```bash
cd vala/
nix run              # Run with Nix
# OR
meson setup build
meson compile -C build
./build/src/pomodoro-timer
```

### Rust
```bash
cd rust/
nix run              # Run with Nix
# OR
cargo run
```

### With Nix (Recommended)
```bash
# Enter development environment
cd vala && nix develop     # Vala development
cd rust && nix develop     # Rust development

# Run applications
cd vala && nix run         # Run Vala version
cd rust && nix run         # Run Rust version

# Run tests
cd vala && nix run .#test  # Run Vala tests
cd rust && nix run .#test  # Run Rust tests
```