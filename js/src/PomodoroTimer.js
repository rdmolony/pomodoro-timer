import { Timer } from './Timer.js';

/**
 * PomodoroTimer extends Timer with Pomodoro-specific functionality
 */
export class PomodoroTimer extends Timer {
  constructor() {
    super();
    
    // Pomodoro settings (in seconds)
    this.workDuration = 25 * 60;      // 25 minutes
    this.shortBreakDuration = 5 * 60;  // 5 minutes
    this.longBreakDuration = 15 * 60;  // 15 minutes
    this.eyeCheckInterval = 40 * 60;   // 40 minutes
    
    // Pomodoro state
    this.sessionCount = 0;
    this.sessionType = 'work'; // 'work', 'shortBreak', 'longBreak'
    this.totalWorkTime = 0;
    this.lastEyeCheck = Date.now();
  }

  /**
   * Start a work session
   */
  startWork() {
    this.sessionType = 'work';
    this.setDuration(this.workDuration);
    this.start();
    this.emit('sessionStarted', { type: 'work', duration: this.workDuration });
  }

  /**
   * Start a short break
   */
  startShortBreak() {
    this.sessionType = 'shortBreak';
    this.setDuration(this.shortBreakDuration);
    this.start();
    this.emit('sessionStarted', { type: 'shortBreak', duration: this.shortBreakDuration });
  }

  /**
   * Start a long break
   */
  startLongBreak() {
    this.sessionType = 'longBreak';
    this.setDuration(this.longBreakDuration);
    this.start();
    this.emit('sessionStarted', { type: 'longBreak', duration: this.longBreakDuration });
  }

  /**
   * Override tick to track work time and eye checks
   */
  tick() {
    const wasRunning = this.running;
    const previousRemaining = this.remaining;
    
    super.tick();
    
    // Track work time
    if (wasRunning && this.sessionType === 'work') {
      this.totalWorkTime++;
      
      // Check for eye break reminder
      const timeSinceEyeCheck = (Date.now() - this.lastEyeCheck) / 1000;
      if (timeSinceEyeCheck >= this.eyeCheckInterval) {
        this.emit('eyeCheckReminder');
        this.lastEyeCheck = Date.now();
      }
    }
    
    // Handle session completion
    if (previousRemaining > 0 && this.remaining === 0) {
      this.handleSessionComplete();
    }
  }

  /**
   * Handle completion of a session
   */
  handleSessionComplete() {
    if (this.sessionType === 'work') {
      this.sessionCount++;
      this.emit('workSessionComplete', { sessionNumber: this.sessionCount });
      
      // Determine next break type
      if (this.sessionCount % 4 === 0) {
        this.emit('suggestLongBreak');
      } else {
        this.emit('suggestShortBreak');
      }
    } else {
      this.emit('breakComplete', { type: this.sessionType });
      this.emit('suggestWork');
    }
  }

  /**
   * Update work duration
   * @param {number} minutes - Duration in minutes
   */
  setWorkDuration(minutes) {
    this.workDuration = minutes * 60;
    this.emit('settingsChanged', { workDuration: this.workDuration });
  }

  /**
   * Update short break duration
   * @param {number} minutes - Duration in minutes
   */
  setShortBreakDuration(minutes) {
    this.shortBreakDuration = minutes * 60;
    this.emit('settingsChanged', { shortBreakDuration: this.shortBreakDuration });
  }

  /**
   * Update long break duration
   * @param {number} minutes - Duration in minutes
   */
  setLongBreakDuration(minutes) {
    this.longBreakDuration = minutes * 60;
    this.emit('settingsChanged', { longBreakDuration: this.longBreakDuration });
  }

  /**
   * Update eye check interval
   * @param {number} minutes - Interval in minutes
   */
  setEyeCheckInterval(minutes) {
    this.eyeCheckInterval = minutes * 60;
    this.emit('settingsChanged', { eyeCheckInterval: this.eyeCheckInterval });
  }

  /**
   * Get current session type
   * @returns {string} Current session type
   */
  getSessionType() {
    return this.sessionType;
  }

  /**
   * Get session count
   * @returns {number} Number of completed work sessions
   */
  getSessionCount() {
    return this.sessionCount;
  }

  /**
   * Get total work time
   * @returns {number} Total work time in seconds
   */
  getTotalWorkTime() {
    return this.totalWorkTime;
  }

  /**
   * Reset all statistics
   */
  resetStats() {
    this.sessionCount = 0;
    this.totalWorkTime = 0;
    this.lastEyeCheck = Date.now();
    this.emit('statsReset');
  }

  /**
   * Get all settings
   * @returns {Object} Current settings
   */
  getSettings() {
    return {
      workDuration: this.workDuration,
      shortBreakDuration: this.shortBreakDuration,
      longBreakDuration: this.longBreakDuration,
      eyeCheckInterval: this.eyeCheckInterval
    };
  }

  /**
   * Get current statistics
   * @returns {Object} Current statistics
   */
  getStats() {
    return {
      sessionCount: this.sessionCount,
      totalWorkTime: this.totalWorkTime,
      sessionType: this.sessionType,
      currentDuration: this.duration,
      remaining: this.remaining,
      isRunning: this.running
    };
  }
}