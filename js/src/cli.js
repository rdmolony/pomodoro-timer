#!/usr/bin/env node

import { PomodoroTimer } from './PomodoroTimer.js';
import readline from 'readline';

/**
 * Simple CLI interface for the Pomodoro Timer
 */
class PomodoorCLI {
  constructor() {
    this.timer = new PomodoroTimer();
    this.rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });
    
    this.setupEventHandlers();
    this.running = true;
  }

  setupEventHandlers() {
    // Timer events
    this.timer.on('tick', (remaining) => {
      this.updateDisplay(remaining);
    });

    this.timer.on('finished', () => {
      console.log('\n🔔 Timer finished!');
    });

    this.timer.on('workSessionComplete', ({ sessionNumber }) => {
      console.log(`\n✅ Work session #${sessionNumber} complete!`);
    });

    this.timer.on('breakComplete', ({ type }) => {
      console.log(`\n☕ ${type === 'shortBreak' ? 'Short' : 'Long'} break complete!`);
    });

    this.timer.on('suggestShortBreak', () => {
      console.log('💡 Time for a short break! Type "short" to start.');
    });

    this.timer.on('suggestLongBreak', () => {
      console.log('💡 Time for a long break! Type "long" to start.');
    });

    this.timer.on('suggestWork', () => {
      console.log('💡 Ready to work? Type "work" to start.');
    });

    this.timer.on('eyeCheckReminder', () => {
      console.log('\n👀 Eye check reminder! Look away from the screen for 20 seconds.');
    });
  }

  updateDisplay(remaining) {
    const time = PomodoroTimer.formatTime(remaining);
    const sessionType = this.timer.getSessionType();
    const status = this.timer.isRunning() ? '▶️ ' : '⏸️ ';
    
    // Clear line and update
    process.stdout.write(`\r${status} ${this.getSessionEmoji(sessionType)} ${time} `);
  }

  getSessionEmoji(type) {
    switch (type) {
      case 'work': return '🍅';
      case 'shortBreak': return '☕';
      case 'longBreak': return '🌴';
      default: return '⏱️';
    }
  }

  showHelp() {
    console.log(`
🍅 Pomodoro Timer CLI

Commands:
  work      - Start a work session (${this.timer.workDuration / 60} minutes)
  short     - Start a short break (${this.timer.shortBreakDuration / 60} minutes)
  long      - Start a long break (${this.timer.longBreakDuration / 60} minutes)
  pause     - Pause the timer
  resume    - Resume the timer
  reset     - Reset the current timer
  stats     - Show statistics
  settings  - Show current settings
  set <type> <minutes> - Change duration (work/short/long/eye)
  clear     - Reset all statistics
  help      - Show this help
  quit      - Exit the program

Current Status:
  Session: ${this.timer.getSessionType()}
  Sessions completed: ${this.timer.getSessionCount()}
  Total work time: ${PomodoroTimer.formatTime(this.timer.getTotalWorkTime())}
`);
  }

  showStats() {
    const stats = this.timer.getStats();
    console.log(`
📊 Statistics:
  Sessions completed: ${stats.sessionCount}
  Total work time: ${PomodoroTimer.formatTime(stats.totalWorkTime)}
  Current session: ${stats.sessionType}
  Time remaining: ${PomodoroTimer.formatTime(stats.remaining)}
  Status: ${stats.isRunning ? 'Running' : 'Paused'}
`);
  }

  showSettings() {
    const settings = this.timer.getSettings();
    console.log(`
⚙️  Settings:
  Work duration: ${settings.workDuration / 60} minutes
  Short break: ${settings.shortBreakDuration / 60} minutes
  Long break: ${settings.longBreakDuration / 60} minutes
  Eye check interval: ${settings.eyeCheckInterval / 60} minutes
`);
  }

  async handleCommand(input) {
    const [command, ...args] = input.trim().toLowerCase().split(' ');

    switch (command) {
      case 'work':
        this.timer.startWork();
        console.log('🍅 Starting work session...');
        break;

      case 'short':
        this.timer.startShortBreak();
        console.log('☕ Starting short break...');
        break;

      case 'long':
        this.timer.startLongBreak();
        console.log('🌴 Starting long break...');
        break;

      case 'pause':
        this.timer.pause();
        console.log('⏸️  Timer paused');
        break;

      case 'resume':
        this.timer.start();
        console.log('▶️  Timer resumed');
        break;

      case 'reset':
        this.timer.reset();
        console.log('🔄 Timer reset');
        break;

      case 'stats':
        this.showStats();
        break;

      case 'settings':
        this.showSettings();
        break;

      case 'set':
        this.handleSet(args);
        break;

      case 'clear':
        this.timer.resetStats();
        console.log('📊 Statistics cleared');
        break;

      case 'help':
      case '?':
        this.showHelp();
        break;

      case 'quit':
      case 'exit':
      case 'q':
        this.running = false;
        break;

      case '':
        // Empty command, just update display
        if (this.timer.isRunning()) {
          this.updateDisplay(this.timer.getRemaining());
        }
        break;

      default:
        console.log(`Unknown command: ${command}. Type "help" for available commands.`);
    }
  }

  handleSet(args) {
    const [type, minutes] = args;
    const mins = parseInt(minutes);

    if (isNaN(mins) || mins <= 0) {
      console.log('❌ Please provide a valid number of minutes');
      return;
    }

    switch (type) {
      case 'work':
        this.timer.setWorkDuration(mins);
        console.log(`✅ Work duration set to ${mins} minutes`);
        break;

      case 'short':
        this.timer.setShortBreakDuration(mins);
        console.log(`✅ Short break duration set to ${mins} minutes`);
        break;

      case 'long':
        this.timer.setLongBreakDuration(mins);
        console.log(`✅ Long break duration set to ${mins} minutes`);
        break;

      case 'eye':
        this.timer.setEyeCheckInterval(mins);
        console.log(`✅ Eye check interval set to ${mins} minutes`);
        break;

      default:
        console.log('❌ Invalid type. Use: work, short, long, or eye');
    }
  }

  async run() {
    console.clear();
    console.log('🍅 Welcome to Pomodoro Timer CLI!');
    console.log('Type "help" for commands or "work" to start.\n');

    this.promptUser();
  }

  promptUser() {
    this.rl.question('> ', async (input) => {
      await this.handleCommand(input);
      
      if (this.running) {
        this.promptUser();
      } else {
        this.cleanup();
      }
    });
  }

  cleanup() {
    console.log('\n👋 Goodbye! Keep being productive!');
    this.timer.destroy();
    this.rl.close();
    process.exit(0);
  }
}

// Run the CLI
const cli = new PomodoorCLI();
cli.run();