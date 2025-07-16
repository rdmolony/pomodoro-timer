# Test Plan for Pomodoro Timer

This plan follows TDD methodology as outlined in CLAUDE.md. Each test should be implemented one at a time, followed by minimal code to make it pass, then refactored if needed.

## Test Coverage Strategy

### 1. Timer Core Logic Tests
- [x] Timer should initialize with zero duration and not running
- [x] Timer should set duration correctly
- [x] Timer should start and emit tick signals
- [x] Timer should track remaining time correctly
- [x] Timer should stop when reaching zero and emit finished signal
- [x] Timer should pause and resume correctly
- [x] Timer should handle multiple start calls gracefully
- [x] Timer should handle pause when not running gracefully

### 2. Notification Manager Tests
- [x] NotificationManager should initialize without errors
- [ ] NotificationManager should show basic notifications
- [ ] NotificationManager should show 20-20-20 notifications with actions
- [ ] NotificationManager should handle sound settings correctly
- [ ] NotificationManager should handle notification errors gracefully
- [ ] NotificationManager should schedule snooze reminders correctly

### 3. Application Tests
- [ ] Application should initialize with correct app ID
- [ ] Application should create settings instance
- [ ] Application should handle activation correctly
- [ ] Application should create window and notification manager
- [ ] Application should handle quit action
- [ ] Application should handle about action
- [ ] Application should provide notification manager access

### 4. Main Window Tests
- [ ] MainWindow should initialize with correct default size
- [ ] MainWindow should create timer instances
- [ ] MainWindow should build UI elements correctly
- [ ] MainWindow should handle timer tick events
- [ ] MainWindow should handle timer finished events
- [ ] MainWindow should handle start/pause button clicks
- [ ] MainWindow should handle reset button clicks
- [ ] MainWindow should update timer display correctly
- [ ] MainWindow should update progress bar correctly
- [ ] MainWindow should handle pomodoro/break cycle correctly
- [ ] MainWindow should handle 20-20-20 rule toggle
- [ ] MainWindow should handle window size persistence

### 5. Integration Tests
- [ ] Complete pomodoro cycle (work → short break → work)
- [ ] Long break after 4 sessions
- [ ] 20-20-20 rule integration with main timer
- [ ] Settings persistence across app restarts
- [ ] Notification integration with timer events

### 6. Edge Cases and Error Handling
- [ ] Timer with invalid duration values
- [ ] Settings file corruption handling
- [ ] Sound system unavailable
- [ ] Notification system unavailable
- [ ] Window resize edge cases
- [ ] Rapid button clicking behavior

## Test Implementation Notes

- Use Vala's built-in testing framework
- Mock external dependencies (GSound, Notify)
- Use GLib.Test for test organization
- Focus on unit tests first, then integration
- Each test should be independent and isolated
- Use descriptive test names that explain the behavior being tested

## Test File Structure
```
tests/
├── test-timer.vala
├── test-notification-manager.vala
├── test-application.vala
├── test-main-window.vala
├── test-integration.vala
└── meson.build
```

## Vala Cross-Directory Compilation Workaround

Due to a meson limitation with cross-directory Vala compilation, we use `custom_target` to copy source files from `src/` to the build directory during test compilation. This approach:

- Keeps the source tree clean (no duplicate files in version control)
- Tests run against the actual source code from `src/`
- Generated copies are placed in the build directory and cleaned up automatically
- Each test suite defines its required source file dependencies via custom_target

Example pattern:
```meson
source_copy = custom_target('copy-source',
  input: '../src/source.vala',
  output: 'source.vala',
  command: ['cp', '@INPUT@', '@OUTPUT@'],
  build_by_default: false
)

test_exe = executable('test-name',
  files('test-file.vala') + [source_copy],
  dependencies: [required_deps],
  install: false,
  build_by_default: false,
  vala_args: ['--target-glib=2.72']
)
```