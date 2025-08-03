import { describe, test, expect, beforeEach, afterEach, jest } from '@jest/globals';
import { PomodoroTimer } from '../../src/PomodoroTimer.js';

describe('PomodoroTimer Unit Tests', () => {
  let timer;

  beforeEach(() => {
    timer = new PomodoroTimer();
    jest.useFakeTimers();
  });

  afterEach(() => {
    timer.destroy();
    jest.useRealTimers();
  });

  describe('Initialization', () => {
    test('should initialize with default pomodoro settings', () => {
      expect(timer.workDuration).toBe(25 * 60);
      expect(timer.shortBreakDuration).toBe(5 * 60);
      expect(timer.longBreakDuration).toBe(15 * 60);
      expect(timer.eyeCheckInterval).toBe(40 * 60);
    });

    test('should initialize session state correctly', () => {
      expect(timer.sessionCount).toBe(0);
      expect(timer.sessionType).toBe('work');
      expect(timer.totalWorkTime).toBe(0);
      expect(timer.lastEyeCheck).toBeCloseTo(Date.now(), -2);
    });

    test('should inherit from Timer', () => {
      expect(timer.start).toBeDefined();
      expect(timer.pause).toBeDefined();
      expect(timer.reset).toBeDefined();
      expect(timer.tick).toBeDefined();
    });
  });

  describe('Session Management', () => {
    test('should start work session correctly', () => {
      const sessionStartedCallback = jest.fn();
      timer.on('sessionStarted', sessionStartedCallback);
      
      timer.startWork();
      
      expect(timer.sessionType).toBe('work');
      expect(timer.getDuration()).toBe(25 * 60);
      expect(timer.isRunning()).toBe(true);
      expect(sessionStartedCallback).toHaveBeenCalledWith({
        type: 'work',
        duration: 25 * 60
      });
    });

    test('should start short break correctly', () => {
      const sessionStartedCallback = jest.fn();
      timer.on('sessionStarted', sessionStartedCallback);
      
      timer.startShortBreak();
      
      expect(timer.sessionType).toBe('shortBreak');
      expect(timer.getDuration()).toBe(5 * 60);
      expect(timer.isRunning()).toBe(true);
      expect(sessionStartedCallback).toHaveBeenCalledWith({
        type: 'shortBreak',
        duration: 5 * 60
      });
    });

    test('should start long break correctly', () => {
      const sessionStartedCallback = jest.fn();
      timer.on('sessionStarted', sessionStartedCallback);
      
      timer.startLongBreak();
      
      expect(timer.sessionType).toBe('longBreak');
      expect(timer.getDuration()).toBe(15 * 60);
      expect(timer.isRunning()).toBe(true);
      expect(sessionStartedCallback).toHaveBeenCalledWith({
        type: 'longBreak',
        duration: 15 * 60
      });
    });
  });

  describe('Work Time Tracking', () => {
    test('should track work time during work sessions', () => {
      timer.startWork();
      
      // Simulate 10 ticks
      for (let i = 0; i < 10; i++) {
        timer.tick();
      }
      
      expect(timer.getTotalWorkTime()).toBe(10);
    });

    test('should not track time during breaks', () => {
      timer.startShortBreak();
      
      // Simulate 5 ticks
      for (let i = 0; i < 5; i++) {
        timer.tick();
      }
      
      expect(timer.getTotalWorkTime()).toBe(0);
    });

    test('should emit eye check reminder after interval', () => {
      const eyeCheckCallback = jest.fn();
      timer.on('eyeCheckReminder', eyeCheckCallback);
      
      // Set short interval for testing
      timer.setEyeCheckInterval(1); // 1 minute
      timer.lastEyeCheck = Date.now() - 60000; // 1 minute ago
      
      timer.startWork();
      timer.tick();
      
      expect(eyeCheckCallback).toHaveBeenCalled();
      expect(timer.lastEyeCheck).toBeCloseTo(Date.now(), -2);
    });
  });

  describe('Session Completion', () => {
    test('should handle work session completion', () => {
      const workCompleteCallback = jest.fn();
      const suggestBreakCallback = jest.fn();
      
      timer.on('workSessionComplete', workCompleteCallback);
      timer.on('suggestShortBreak', suggestBreakCallback);
      
      timer.setWorkDuration(1/60); // 1 second for testing
      timer.startWork();
      timer.tick(); // Complete the session
      
      expect(workCompleteCallback).toHaveBeenCalledWith({ sessionNumber: 1 });
      expect(suggestBreakCallback).toHaveBeenCalled();
      expect(timer.sessionCount).toBe(1);
    });

    test('should suggest long break after 4 work sessions', () => {
      const suggestLongBreakCallback = jest.fn();
      timer.on('suggestLongBreak', suggestLongBreakCallback);
      
      // Complete 4 work sessions
      timer.sessionCount = 3; // Set to 3 so next completion is 4th
      timer.setWorkDuration(1/60);
      timer.startWork();
      timer.tick();
      
      expect(suggestLongBreakCallback).toHaveBeenCalled();
    });

    test('should handle break completion', () => {
      const breakCompleteCallback = jest.fn();
      const suggestWorkCallback = jest.fn();
      
      timer.on('breakComplete', breakCompleteCallback);
      timer.on('suggestWork', suggestWorkCallback);
      
      timer.setShortBreakDuration(1/60);
      timer.startShortBreak();
      timer.tick();
      
      expect(breakCompleteCallback).toHaveBeenCalledWith({ type: 'shortBreak' });
      expect(suggestWorkCallback).toHaveBeenCalled();
    });
  });

  describe('Settings Management', () => {
    test('should update work duration', () => {
      const settingsCallback = jest.fn();
      timer.on('settingsChanged', settingsCallback);
      
      timer.setWorkDuration(30);
      expect(timer.workDuration).toBe(30 * 60);
      expect(settingsCallback).toHaveBeenCalledWith({ workDuration: 30 * 60 });
    });

    test('should update short break duration', () => {
      const settingsCallback = jest.fn();
      timer.on('settingsChanged', settingsCallback);
      
      timer.setShortBreakDuration(10);
      expect(timer.shortBreakDuration).toBe(10 * 60);
      expect(settingsCallback).toHaveBeenCalledWith({ shortBreakDuration: 10 * 60 });
    });

    test('should update long break duration', () => {
      const settingsCallback = jest.fn();
      timer.on('settingsChanged', settingsCallback);
      
      timer.setLongBreakDuration(20);
      expect(timer.longBreakDuration).toBe(20 * 60);
      expect(settingsCallback).toHaveBeenCalledWith({ longBreakDuration: 20 * 60 });
    });

    test('should update eye check interval', () => {
      const settingsCallback = jest.fn();
      timer.on('settingsChanged', settingsCallback);
      
      timer.setEyeCheckInterval(30);
      expect(timer.eyeCheckInterval).toBe(30 * 60);
      expect(settingsCallback).toHaveBeenCalledWith({ eyeCheckInterval: 30 * 60 });
    });

    test('should get all settings', () => {
      const settings = timer.getSettings();
      expect(settings).toEqual({
        workDuration: 25 * 60,
        shortBreakDuration: 5 * 60,
        longBreakDuration: 15 * 60,
        eyeCheckInterval: 40 * 60
      });
    });
  });

  describe('Statistics', () => {
    test('should get current statistics', () => {
      timer.sessionCount = 3;
      timer.totalWorkTime = 4500; // 75 minutes
      timer.setDuration(1500);
      timer.setRemaining(1200);
      
      const stats = timer.getStats();
      expect(stats).toEqual({
        sessionCount: 3,
        totalWorkTime: 4500,
        sessionType: 'work',
        currentDuration: 1500,
        remaining: 1200,
        isRunning: false
      });
    });

    test('should reset statistics', () => {
      const resetCallback = jest.fn();
      timer.on('statsReset', resetCallback);
      
      timer.sessionCount = 5;
      timer.totalWorkTime = 7500;
      
      timer.resetStats();
      
      expect(timer.sessionCount).toBe(0);
      expect(timer.totalWorkTime).toBe(0);
      expect(timer.lastEyeCheck).toBeCloseTo(Date.now(), -2);
      expect(resetCallback).toHaveBeenCalled();
    });
  });

  describe('Getters', () => {
    test('should get session type', () => {
      expect(timer.getSessionType()).toBe('work');
      
      timer.startShortBreak();
      expect(timer.getSessionType()).toBe('shortBreak');
      
      timer.startLongBreak();
      expect(timer.getSessionType()).toBe('longBreak');
    });

    test('should get session count', () => {
      expect(timer.getSessionCount()).toBe(0);
      
      timer.sessionCount = 5;
      expect(timer.getSessionCount()).toBe(5);
    });

    test('should get total work time', () => {
      expect(timer.getTotalWorkTime()).toBe(0);
      
      timer.totalWorkTime = 3600;
      expect(timer.getTotalWorkTime()).toBe(3600);
    });
  });

  describe('Integration with Timer Base Class', () => {
    test('should maintain timer functionality', () => {
      timer.setDuration(100);
      timer.start();
      
      expect(timer.isRunning()).toBe(true);
      expect(timer.getRemaining()).toBe(100);
      
      timer.pause();
      expect(timer.isRunning()).toBe(false);
      
      timer.reset();
      expect(timer.getRemaining()).toBe(100);
    });

    test('should emit both timer and pomodoro events', () => {
      const tickCallback = jest.fn();
      const finishedCallback = jest.fn();
      const workCompleteCallback = jest.fn();
      
      timer.on('tick', tickCallback);
      timer.on('finished', finishedCallback);
      timer.on('workSessionComplete', workCompleteCallback);
      
      timer.setWorkDuration(1/60);
      timer.startWork();
      timer.tick();
      
      expect(tickCallback).toHaveBeenCalled();
      expect(finishedCallback).toHaveBeenCalled();
      expect(workCompleteCallback).toHaveBeenCalled();
    });
  });

  describe('Edge Cases', () => {
    test('should handle session completion at exactly 4 sessions', () => {
      const suggestLongBreakCallback = jest.fn();
      const suggestShortBreakCallback = jest.fn();
      
      timer.on('suggestLongBreak', suggestLongBreakCallback);
      timer.on('suggestShortBreak', suggestShortBreakCallback);
      
      // Test sessions 1-3 suggest short break
      for (let i = 0; i < 3; i++) {
        timer.sessionCount = i;
        timer.setWorkDuration(1/60);
        timer.startWork();
        timer.tick();
      }
      
      expect(suggestShortBreakCallback).toHaveBeenCalledTimes(3);
      expect(suggestLongBreakCallback).not.toHaveBeenCalled();
      
      // 4th session should suggest long break
      timer.sessionCount = 3;
      timer.setWorkDuration(1/60);
      timer.startWork();
      timer.tick();
      
      expect(suggestLongBreakCallback).toHaveBeenCalled();
    });

    test('should handle eye check with exact timing', () => {
      const eyeCheckCallback = jest.fn();
      timer.on('eyeCheckReminder', eyeCheckCallback);
      
      timer.setEyeCheckInterval(1); // 1 minute
      timer.lastEyeCheck = Date.now() - 60000; // Exactly 1 minute ago
      
      timer.startWork();
      timer.tick();
      
      expect(eyeCheckCallback).toHaveBeenCalled();
    });

    test('should not emit eye check during breaks', () => {
      const eyeCheckCallback = jest.fn();
      timer.on('eyeCheckReminder', eyeCheckCallback);
      
      timer.setEyeCheckInterval(1/60); // 1 second
      timer.lastEyeCheck = Date.now() - 2000; // 2 seconds ago
      
      timer.startShortBreak();
      timer.tick();
      
      expect(eyeCheckCallback).not.toHaveBeenCalled();
    });
  });
});