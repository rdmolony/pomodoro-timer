# Pomodoro Timer Rust/Relm4 TDD Plan

## Overview
This plan follows Kent Beck's Test-Driven Development methodology for transpiling the current Vala/GTK Pomodoro timer to Rust using the Relm4 framework. Each test represents a small, focused increment of functionality following the Red → Green → Refactor cycle.

## TDD Methodology
- Write the simplest failing test first
- Implement only enough code to make the test pass
- Refactor when tests are passing
- Commit structural changes separately from behavioral changes
- Run all tests after each change using `cargo test`

## Rust Project Setup
- Use `cargo new pomodoro-timer-rust` to create new Rust project
- Configure `Cargo.toml` with Relm4 dependencies
- Use `cargo test` to run all tests
- Use `cargo test test_name` to run specific tests
- Use `cargo test --lib` for unit tests, `cargo test --test` for integration tests

## Test Implementation Order

### Phase 1: Core Timer Logic (Foundation)

## Test 1: [x] Timer should initialize with zero duration
**Focus**: Basic timer struct creation
**Implementation**: Create Timer struct with duration field
**Expected**: Timer::new() creates timer with 0 duration

## Test 2: [x] Timer should set duration correctly
**Focus**: Duration setting functionality
**Implementation**: Add set_duration method
**Expected**: timer.set_duration(1500) sets duration to 1500 seconds

## Test 3: [x] Timer should report remaining time
**Focus**: Time remaining getter
**Implementation**: Add get_remaining method
**Expected**: timer.get_remaining() returns current remaining time

## Test 4: [x] Timer should report total duration
**Focus**: Total duration getter
**Implementation**: Add get_total_duration method
**Expected**: timer.get_total_duration() returns original duration

## Test 5: [x] Timer should start countdown
**Focus**: Timer start functionality
**Implementation**: Add start method and running state
**Expected**: timer.start() changes state to running

## Test 6: [x] Timer should pause countdown
**Focus**: Timer pause functionality
**Implementation**: Add pause method
**Expected**: timer.pause() changes state to paused

## Test 7: [x] Timer should reset to original duration
**Focus**: Timer reset functionality
**Implementation**: Add reset method
**Expected**: timer.reset() restores original duration

## Test 8: [x] Timer should emit tick events
**Focus**: Timer tick signal/callback
**Implementation**: Add tick event mechanism
**Expected**: Running timer emits tick events with remaining time

## Test 9: [x] Timer should emit finished event when complete
**Focus**: Timer completion signal
**Implementation**: Add finished event mechanism
**Expected**: Timer emits finished event when countdown reaches zero

## Test 10: [x] Timer should stop running when finished
**Focus**: Timer auto-stop on completion
**Implementation**: Update timer state on finish
**Expected**: Timer state becomes stopped when finished

### Phase 2: Timer Component (Relm4 Integration)

## Test 11: [x] TimerModel should initialize with default state
**Focus**: Relm4 component model creation
**Implementation**: Create TimerModel struct with Component trait
**Expected**: TimerModel::init() creates proper initial state

## Test 12: [x] TimerModel should handle Start message
**Focus**: Message-driven state updates
**Implementation**: Add update method handling TimerMsg::Start
**Expected**: Start message transitions timer to running state

## Test 13: [x] TimerModel should handle Pause message
**Focus**: Message handling for pause
**Implementation**: Handle TimerMsg::Pause in update
**Expected**: Pause message transitions timer to paused state

## Test 14: [x] TimerModel should handle Reset message
**Focus**: Message handling for reset
**Implementation**: Handle TimerMsg::Reset in update
**Expected**: Reset message restores timer to initial state

## Test 15: [x] TimerModel should handle Tick message
**Focus**: Message handling for timer updates
**Implementation**: Handle TimerMsg::Tick in update
**Expected**: Tick message updates remaining time

## Test 16: [x] TimerModel should create timer display UI
**Focus**: Declarative UI creation
**Implementation**: Create view method returning GTK widgets
**Expected**: view() returns properly structured timer UI

## Test 17: [x] TimerModel should update UI when state changes
**Focus**: Reactive UI updates
**Implementation**: Update view based on model state
**Expected**: UI reflects current timer state and remaining time

## Test 18: [x] TimerModel should handle button clicks
**Focus**: User interaction handling
**Implementation**: Connect button signals to message sending
**Expected**: Button clicks send appropriate messages

### Phase 3: Eye Check Component

## Test 19: [x] EyeCheckModel should initialize hidden
**Focus**: Eye check dialog initial state
**Implementation**: Create EyeCheckModel with visibility state
**Expected**: EyeCheckModel::init() creates hidden dialog

## Test 20: [x] EyeCheckModel should show dialog on Show message
**Focus**: Dialog visibility control
**Implementation**: Handle EyeCheckMsg::Show in update
**Expected**: Show message makes dialog visible

## Test 21: [ ] EyeCheckModel should create fullscreen dialog UI
**Focus**: Eye check dialog UI creation
**Implementation**: Create view method for fullscreen dialog
**Expected**: view() returns fullscreen dialog with proper layout

## Test 22: [ ] EyeCheckModel should handle dismiss action
**Focus**: Dialog dismissal handling
**Implementation**: Handle EyeCheckMsg::Dismiss in update
**Expected**: Dismiss message hides dialog and sends notification

## Test 23: [ ] EyeCheckModel should handle snooze action
**Focus**: Dialog snooze handling
**Implementation**: Handle EyeCheckMsg::Snooze in update
**Expected**: Snooze message hides dialog and schedules reminder

## Test 24: [ ] EyeCheckModel should handle escape key
**Focus**: Keyboard interaction
**Implementation**: Add key event handling
**Expected**: Escape key dismisses dialog

### Phase 4: Settings Component

## Test 25: [ ] SettingsModel should initialize with defaults
**Focus**: Settings component initialization
**Implementation**: Create SettingsModel with default values
**Expected**: SettingsModel::init() creates proper defaults

## Test 26: [ ] SettingsModel should handle settings changes
**Focus**: Settings update handling
**Implementation**: Handle SettingsMsg variants in update
**Expected**: Settings messages update model state

## Test 27: [ ] SettingsModel should persist settings
**Focus**: Settings persistence
**Implementation**: Add settings save/load functionality
**Expected**: Settings persist across application restarts

## Test 28: [ ] SettingsModel should create settings UI
**Focus**: Settings UI creation
**Implementation**: Create view method for settings panel
**Expected**: view() returns properly structured settings UI

### Phase 5: Application Integration

## Test 29: [ ] AppModel should initialize with all components
**Focus**: Root application model
**Implementation**: Create AppModel with component controllers
**Expected**: AppModel::init() creates and connects all components

## Test 30: [ ] AppModel should handle inter-component communication
**Focus**: Component message passing
**Implementation**: Add component communication mechanisms
**Expected**: Components can send messages to each other

## Test 31: [ ] AppModel should create main window UI
**Focus**: Main application UI
**Implementation**: Create view method composing all components
**Expected**: view() returns complete application window

## Test 32: [ ] AppModel should handle 20-20-20 timer integration
**Focus**: Eye check scheduling
**Implementation**: Add 20-20-20 timer logic
**Expected**: Eye check dialogs appear every 20 minutes when enabled

## Test 33: [ ] AppModel should handle session management
**Focus**: Pomodoro session tracking
**Implementation**: Add session counting and break scheduling
**Expected**: Application tracks sessions and schedules breaks

## Test 34: [ ] AppModel should handle notifications
**Focus**: System notification integration
**Implementation**: Add notification service
**Expected**: Application shows system notifications for events

## Test 35: [ ] AppModel should handle window state persistence
**Focus**: Window state management
**Implementation**: Add window size/position persistence
**Expected**: Window state persists across application restarts

### Phase 6: Final Integration

## Test 36: [ ] Application should handle complete pomodoro workflow
**Focus**: End-to-end workflow testing
**Implementation**: Integration test covering full user journey
**Expected**: Complete pomodoro cycle works as expected

## Test 37: [ ] Application should handle multi-monitor eye check
**Focus**: Multi-monitor support
**Implementation**: Add multi-monitor eye check display
**Expected**: Eye check dialogs appear on all monitors

## Test 38: [ ] Application should handle graceful shutdown
**Focus**: Application cleanup
**Implementation**: Add proper shutdown handling
**Expected**: Application cleans up resources on exit

## Expected Cargo.toml Dependencies
```toml
[package]
name = "pomodoro-timer-rust"
version = "0.1.0"
edition = "2021"

[dependencies]
relm4 = "0.8"
relm4-components = "0.8"
gtk = { version = "0.8", package = "gtk4" }
adw = { version = "0.6", package = "libadwaita" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
dirs = "5.0"
notify-rust = "4.0"

[dev-dependencies]
tokio-test = "0.4"
```

## Rust Project Structure
```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library root
├── timer/
│   ├── mod.rs           # Timer module
│   └── core.rs          # Core timer logic
├── components/
│   ├── mod.rs           # Components module
│   ├── timer.rs         # Timer component
│   ├── eye_check.rs     # Eye check component
│   └── settings.rs      # Settings component
└── app.rs               # Main application model

tests/
├── integration/
│   ├── mod.rs
│   └── workflow.rs      # End-to-end tests
└── unit/
    ├── mod.rs
    └── timer.rs         # Unit tests
```

## Test Commands
- `cargo test` - Run all tests
- `cargo test timer` - Run timer-related tests
- `cargo test --lib` - Run unit tests only
- `cargo test --test integration` - Run integration tests only
- `cargo test -- --nocapture` - Show println! output during tests

## Notes
- Each test should be implemented one at a time
- Only write enough code to make the current test pass
- Refactor only when tests are passing
- Use meaningful test names that describe behavior
- Run `cargo test` after each implementation step
- Commit structural changes separately from behavioral changes
- Use `#[cfg(test)]` modules for unit tests
- Use `tests/` directory for integration tests