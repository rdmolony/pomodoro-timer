import { describe, test, expect, beforeEach, afterEach, jest } from '@jest/globals';
import { Timer } from '../../src/Timer.js';

describe('Timer Unit Tests', () => {
  let timer;

  beforeEach(() => {
    timer = new Timer();
    jest.useFakeTimers();
  });

  afterEach(() => {
    timer.destroy();
    jest.useRealTimers();
  });

  describe('Initialization', () => {
    test('should initialize with default values', () => {
      expect(timer.getDuration()).toBe(0);
      expect(timer.getRemaining()).toBe(0);
      expect(timer.isRunning()).toBe(false);
      expect(timer.intervalId).toBeNull();
    });

    test('should be an EventEmitter', () => {
      expect(timer.on).toBeDefined();
      expect(timer.emit).toBeDefined();
      expect(timer.removeAllListeners).toBeDefined();
    });
  });

  describe('Duration Management', () => {
    test('should set duration correctly', () => {
      timer.setDuration(1500);
      expect(timer.getDuration()).toBe(1500);
      expect(timer.getRemaining()).toBe(1500);
      expect(timer.getTotalDuration()).toBe(1500);
    });

    test('should emit durationChanged event when duration is set', () => {
      const callback = jest.fn();
      timer.on('durationChanged', callback);
      
      timer.setDuration(300);
      expect(callback).toHaveBeenCalledWith(300);
    });

    test('should throw error for negative duration', () => {
      expect(() => timer.setDuration(-100)).toThrow('Duration cannot be negative');
    });
  });

  describe('Timer Controls', () => {
    beforeEach(() => {
      timer.setDuration(60);
    });

    test('should start timer when not running', () => {
      const startCallback = jest.fn();
      timer.on('started', startCallback);
      
      timer.start();
      expect(timer.isRunning()).toBe(true);
      expect(startCallback).toHaveBeenCalled();
      expect(timer.intervalId).not.toBeNull();
    });

    test('should not start timer when already running', () => {
      timer.start();
      const intervalId = timer.intervalId;
      
      timer.start(); // Second start
      expect(timer.intervalId).toBe(intervalId); // Same interval
    });

    test('should not start timer when remaining is 0', () => {
      timer.setDuration(0);
      timer.start();
      expect(timer.isRunning()).toBe(false);
      expect(timer.intervalId).toBeNull();
    });

    test('should pause timer when running', () => {
      const pauseCallback = jest.fn();
      timer.on('paused', pauseCallback);
      
      timer.start();
      timer.pause();
      
      expect(timer.isRunning()).toBe(false);
      expect(pauseCallback).toHaveBeenCalled();
      expect(timer.intervalId).toBeNull();
    });

    test('should handle pause when not running', () => {
      const pauseCallback = jest.fn();
      timer.on('paused', pauseCallback);
      
      timer.pause();
      expect(pauseCallback).not.toHaveBeenCalled();
    });

    test('should reset timer correctly', () => {
      const resetCallback = jest.fn();
      timer.on('reset', resetCallback);
      
      timer.start();
      jest.advanceTimersByTime(5000); // Advance 5 seconds
      
      timer.reset();
      expect(timer.isRunning()).toBe(false);
      expect(timer.getRemaining()).toBe(60);
      expect(resetCallback).toHaveBeenCalled();
    });

    test('stop should work as alias for pause', () => {
      timer.start();
      timer.stop();
      expect(timer.isRunning()).toBe(false);
    });
  });

  describe('Tick Functionality', () => {
    beforeEach(() => {
      timer.setDuration(5);
    });

    test('should decrement remaining time on tick when running', () => {
      timer.start();
      timer.tick();
      expect(timer.getRemaining()).toBe(4);
    });

    test('should emit tick event with remaining time', () => {
      const tickCallback = jest.fn();
      timer.on('tick', tickCallback);
      
      timer.start();
      timer.tick();
      expect(tickCallback).toHaveBeenCalledWith(4);
    });

    test('should not tick when not running', () => {
      timer.tick();
      expect(timer.getRemaining()).toBe(5);
    });

    test('should stop and emit finished when reaching zero', () => {
      const finishedCallback = jest.fn();
      timer.on('finished', finishedCallback);
      
      timer.start();
      
      // Tick down to zero
      for (let i = 0; i < 5; i++) {
        timer.tick();
      }
      
      expect(timer.getRemaining()).toBe(0);
      expect(timer.isRunning()).toBe(false);
      expect(finishedCallback).toHaveBeenCalled();
    });

    test('should not tick below zero', () => {
      timer.setDuration(1);
      timer.start();
      
      timer.tick(); // 1 -> 0
      timer.tick(); // Should do nothing
      
      expect(timer.getRemaining()).toBe(0);
    });

    test('should automatically tick with interval', () => {
      const tickCallback = jest.fn();
      timer.on('tick', tickCallback);
      
      timer.start();
      
      jest.advanceTimersByTime(3000); // 3 seconds
      
      expect(tickCallback).toHaveBeenCalledTimes(3);
      expect(timer.getRemaining()).toBe(2);
    });
  });

  describe('Manual Time Setting', () => {
    test('should set remaining time correctly', () => {
      timer.setDuration(100);
      timer.setRemaining(50);
      expect(timer.getRemaining()).toBe(50);
    });

    test('should emit remainingChanged event', () => {
      const callback = jest.fn();
      timer.on('remainingChanged', callback);
      
      timer.setDuration(100);
      timer.setRemaining(75);
      expect(callback).toHaveBeenCalledWith(75);
    });

    test('should throw error for negative remaining time', () => {
      timer.setDuration(100);
      expect(() => timer.setRemaining(-10)).toThrow('Invalid remaining time');
    });

    test('should throw error for remaining time exceeding duration', () => {
      timer.setDuration(100);
      expect(() => timer.setRemaining(150)).toThrow('Invalid remaining time');
    });
  });

  describe('Resource Cleanup', () => {
    test('should clean up interval on destroy', () => {
      timer.setDuration(60);
      timer.start();
      
      const intervalId = timer.intervalId;
      timer.destroy();
      
      expect(timer.intervalId).toBeNull();
      expect(timer.listenerCount('tick')).toBe(0);
    });

    test('should handle destroy when not running', () => {
      expect(() => timer.destroy()).not.toThrow();
    });
  });

  describe('Static Methods', () => {
    test('should format time correctly', () => {
      expect(Timer.formatTime(0)).toBe('00:00');
      expect(Timer.formatTime(59)).toBe('00:59');
      expect(Timer.formatTime(60)).toBe('01:00');
      expect(Timer.formatTime(90)).toBe('01:30');
      expect(Timer.formatTime(3599)).toBe('59:59');
      expect(Timer.formatTime(3600)).toBe('60:00');
    });
  });

  describe('Edge Cases', () => {
    test('should handle zero duration timer', () => {
      timer.setDuration(0);
      timer.start();
      expect(timer.isRunning()).toBe(false);
    });

    test('should handle very large duration', () => {
      const largeDuration = 86400; // 24 hours
      timer.setDuration(largeDuration);
      expect(timer.getDuration()).toBe(largeDuration);
      expect(timer.getRemaining()).toBe(largeDuration);
    });

    test('should maintain callbacks after reset', () => {
      const tickCallback = jest.fn();
      timer.on('tick', tickCallback);
      
      timer.setDuration(10);
      timer.start();
      timer.tick();
      
      timer.reset();
      timer.start();
      timer.tick();
      
      expect(tickCallback).toHaveBeenCalledTimes(2);
    });

    test('should handle rapid start/stop cycles', () => {
      timer.setDuration(100);
      
      for (let i = 0; i < 10; i++) {
        timer.start();
        timer.pause();
      }
      
      expect(timer.isRunning()).toBe(false);
      expect(timer.getRemaining()).toBe(100);
    });
  });
});