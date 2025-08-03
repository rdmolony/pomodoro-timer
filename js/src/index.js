/**
 * Pomodoro Timer - JavaScript Implementation
 * 
 * Entry point that exports the main classes and utilities
 */

export { Timer } from './Timer.js';
export { PomodoroTimer } from './PomodoroTimer.js';

// Re-export static utility methods
export const formatTime = (seconds) => {
  const minutes = Math.floor(seconds / 60);
  const secs = seconds % 60;
  return `${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
};

// Default export for convenience
export { PomodoroTimer as default } from './PomodoroTimer.js';