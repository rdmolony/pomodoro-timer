import { EventEmitter } from 'eventemitter3';

/**
 * Core Timer class - Pure business logic for a countdown timer
 * Demonstrates separation of concerns with event-based architecture
 */
export class Timer extends EventEmitter {
  constructor() {
    super();
    this.duration = 0;        // Total duration in seconds
    this.remaining = 0;       // Current remaining time in seconds
    this.running = false;     // Whether the timer is actively counting down
    this.intervalId = null;   // Reference to the interval timer
  }

  /**
   * Get the total duration of the timer
   * @returns {number} Duration in seconds
   */
  getDuration() {
    return this.duration;
  }

  /**
   * Set the timer duration
   * @param {number} duration - Duration in seconds
   */
  setDuration(duration) {
    if (duration < 0) {
      throw new Error('Duration cannot be negative');
    }
    this.duration = duration;
    this.remaining = duration;
    this.emit('durationChanged', duration);
  }

  /**
   * Get remaining time
   * @returns {number} Remaining seconds
   */
  getRemaining() {
    return this.remaining;
  }

  /**
   * Get total duration (alias for getDuration)
   * @returns {number} Total duration in seconds
   */
  getTotalDuration() {
    return this.duration;
  }

  /**
   * Check if timer is running
   * @returns {boolean} Running state
   */
  isRunning() {
    return this.running;
  }

  /**
   * Start the timer
   */
  start() {
    if (!this.running && this.remaining > 0) {
      this.running = true;
      this.emit('started');
      
      // Set up interval for ticking
      this.intervalId = setInterval(() => {
        this.tick();
      }, 1000);
    }
  }

  /**
   * Pause the timer
   */
  pause() {
    if (this.running) {
      this.running = false;
      if (this.intervalId) {
        clearInterval(this.intervalId);
        this.intervalId = null;
      }
      this.emit('paused');
    }
  }

  /**
   * Reset the timer to original duration
   */
  reset() {
    this.pause();
    this.remaining = this.duration;
    this.emit('reset');
  }

  /**
   * Stop the timer (alias for pause)
   */
  stop() {
    this.pause();
  }

  /**
   * Advance the timer by one second
   */
  tick() {
    if (this.running && this.remaining > 0) {
      this.remaining--;
      this.emit('tick', this.remaining);
      
      if (this.remaining === 0) {
        this.running = false;
        if (this.intervalId) {
          clearInterval(this.intervalId);
          this.intervalId = null;
        }
        this.emit('finished');
      }
    }
  }

  /**
   * Manually set remaining time (for testing)
   * @param {number} seconds - Remaining seconds
   */
  setRemaining(seconds) {
    if (seconds < 0 || seconds > this.duration) {
      throw new Error('Invalid remaining time');
    }
    this.remaining = seconds;
    this.emit('remainingChanged', seconds);
  }

  /**
   * Clean up resources
   */
  destroy() {
    if (this.intervalId) {
      clearInterval(this.intervalId);
      this.intervalId = null;
    }
    this.removeAllListeners();
  }

  /**
   * Format time as MM:SS
   * @param {number} seconds - Time in seconds
   * @returns {string} Formatted time string
   */
  static formatTime(seconds) {
    const minutes = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
}