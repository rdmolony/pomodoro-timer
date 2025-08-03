# Pomodoro Timer - JavaScript Implementation

A comprehensive Pomodoro Timer implementation in JavaScript with event-driven architecture, full test coverage, and multiple interfaces.

## Features

- **Core Timer Functionality**
  - Countdown timer with start, pause, resume, and reset
  - Event-driven architecture using EventEmitter
  - Precise time tracking with automatic intervals

- **Pomodoro-Specific Features**
  - Work sessions (25 minutes default)
  - Short breaks (5 minutes default)
  - Long breaks (15 minutes default)
  - Automatic session counting
  - Eye check reminders (40 minutes default)
  - Break suggestions after work sessions

- **Multiple Interfaces**
  - Command-line interface (CLI)
  - Web interface with modern UI
  - Programmatic API for integration

## Installation

```bash
cd js
npm install
```

## Usage

### Command-Line Interface

```bash
# Start the CLI
npm start

# Or directly
node src/cli.js
```

CLI Commands:
- `work` - Start a work session
- `short` - Start a short break
- `long` - Start a long break
- `pause` - Pause the timer
- `resume` - Resume the timer
- `reset` - Reset the current timer
- `stats` - Show statistics
- `settings` - Show current settings
- `set <type> <minutes>` - Change duration (work/short/long/eye)
- `clear` - Reset all statistics
- `help` - Show help
- `quit` - Exit the program

### Web Interface

```bash
# Start the web server
npm run serve

# Open http://localhost:8080 in your browser
```

The web interface includes:
- Visual countdown display
- One-click session controls
- Session statistics
- Customizable durations
- Browser notifications
- Sound alerts
- Persistent settings via localStorage

### Programmatic Usage

```javascript
import { PomodoroTimer } from './src/PomodoroTimer.js';

const timer = new PomodoroTimer();

// Event listeners
timer.on('tick', (remaining) => {
  console.log(`Time remaining: ${remaining} seconds`);
});

timer.on('finished', () => {
  console.log('Timer finished!');
});

timer.on('workSessionComplete', ({ sessionNumber }) => {
  console.log(`Completed work session #${sessionNumber}`);
});

// Start a work session
timer.startWork();

// Start breaks
timer.startShortBreak();
timer.startLongBreak();

// Control timer
timer.pause();
timer.start(); // Resume
timer.reset();

// Configure durations (in minutes)
timer.setWorkDuration(30);
timer.setShortBreakDuration(10);
timer.setLongBreakDuration(20);
timer.setEyeCheckInterval(45);

// Get statistics
const stats = timer.getStats();
console.log(`Sessions: ${stats.sessionCount}`);
console.log(`Total work time: ${stats.totalWorkTime} seconds`);
```

## Testing

```bash
# Run all tests with coverage
npm test

# Run tests in watch mode
npm run test:watch

# Run specific test suites
npm run test:unit        # Unit tests only
npm run test:integration # Integration tests only

# Run performance benchmarks
npm run test:performance
```

### Test Coverage

The implementation includes comprehensive test coverage:
- **Unit Tests**: Core Timer and PomodoroTimer functionality
- **Integration Tests**: Complete pomodoro flows and scenarios
- **Property-Based Tests**: Using fast-check for edge cases
- **Performance Tests**: Benchmarks using the benchmark library

## Architecture

### Core Components

1. **Timer.js**
   - Base timer class with countdown functionality
   - Event-driven with EventEmitter3
   - Automatic interval management
   - Resource cleanup on destroy

2. **PomodoroTimer.js**
   - Extends Timer with pomodoro-specific features
   - Session management (work/break cycles)
   - Work time tracking
   - Eye check reminders
   - Statistics tracking

3. **cli.js**
   - Interactive command-line interface
   - Real-time display updates
   - Persistent session management
   - Settings configuration

4. **index.html**
   - Modern web interface
   - Responsive design
   - Local storage for settings
   - Browser notifications
   - Audio alerts

### Event System

The timer emits various events for state changes:

```javascript
// Timer events
'started'        // Timer started
'paused'         // Timer paused
'reset'          // Timer reset
'tick'           // Every second (with remaining time)
'finished'       // Timer reached zero
'durationChanged' // Duration was changed
'remainingChanged' // Remaining time manually set

// Pomodoro events
'sessionStarted'      // Any session started
'workSessionComplete' // Work session finished
'breakComplete'       // Break session finished
'suggestShortBreak'   // After work (not 4th session)
'suggestLongBreak'    // After 4th work session
'suggestWork'         // After any break
'eyeCheckReminder'    // Eye check interval reached
'settingsChanged'     // Settings were updated
'statsReset'          // Statistics were cleared
```

## Configuration

Default durations:
- Work session: 25 minutes
- Short break: 5 minutes
- Long break: 15 minutes
- Eye check interval: 40 minutes

All durations can be customized through the settings methods or UI.

## Development

### Project Structure

```
js/
├── src/
│   ├── Timer.js          # Core timer class
│   ├── PomodoroTimer.js  # Pomodoro-specific features
│   └── cli.js           # CLI interface
├── tests/
│   ├── unit/            # Unit tests
│   ├── integration/     # Integration tests
│   └── performance/     # Performance benchmarks
├── public/
│   └── index.html       # Web interface
├── package.json         # Project configuration
└── README.md           # This file
```

### Code Style

- ES6+ modules
- Event-driven architecture
- Comprehensive JSDoc comments
- Functional programming where appropriate
- Proper error handling and validation

## License

MIT