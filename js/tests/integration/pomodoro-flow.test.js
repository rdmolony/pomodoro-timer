import { describe, test, expect, beforeEach, afterEach, jest } from '@jest/globals';
import { PomodoroTimer } from '../../src/PomodoroTimer.js';

describe('Pomodoro Flow Integration Tests', () => {
  let timer;
  let events;

  beforeEach(() => {
    timer = new PomodoroTimer();
    events = [];
    jest.useFakeTimers();
    
    // Capture all events for verification
    const eventTypes = [
      'started', 'paused', 'reset', 'tick', 'finished',
      'sessionStarted', 'workSessionComplete', 'breakComplete',
      'suggestShortBreak', 'suggestLongBreak', 'suggestWork',
      'eyeCheckReminder', 'settingsChanged', 'statsReset'
    ];
    
    eventTypes.forEach(event => {
      timer.on(event, (data) => {
        events.push({ type: event, data, timestamp: Date.now() });
      });
    });
  });

  afterEach(() => {
    timer.destroy();
    jest.useRealTimers();
  });

  describe('Complete Pomodoro Cycle', () => {
    test('should complete full pomodoro cycle with 4 work sessions', () => {
      // Set short durations for testing
      timer.setWorkDuration(5 / 60); // 5 seconds
      timer.setShortBreakDuration(2 / 60); // 2 seconds
      timer.setLongBreakDuration(3 / 60); // 3 seconds

      // Complete 4 pomodoro cycles
      for (let cycle = 0; cycle < 4; cycle++) {
        events = []; // Clear events for each cycle
        
        // Start work session
        timer.startWork();
        expect(timer.getSessionType()).toBe('work');
        expect(timer.isRunning()).toBe(true);
        
        // Complete work session
        jest.advanceTimersByTime(5000);
        
        // Verify work completion
        expect(events.some(e => e.type === 'workSessionComplete')).toBe(true);
        expect(timer.getSessionCount()).toBe(cycle + 1);
        
        // Check break suggestion
        if (cycle < 3) {
          expect(events.some(e => e.type === 'suggestShortBreak')).toBe(true);
          
          // Start short break
          timer.startShortBreak();
          jest.advanceTimersByTime(2000);
          
          expect(events.some(e => e.type === 'breakComplete')).toBe(true);
          expect(events.some(e => e.type === 'suggestWork')).toBe(true);
        } else {
          // 4th session should suggest long break
          expect(events.some(e => e.type === 'suggestLongBreak')).toBe(true);
          
          // Start long break
          timer.startLongBreak();
          jest.advanceTimersByTime(3000);
          
          expect(events.some(e => e.type === 'breakComplete')).toBe(true);
        }
      }

      expect(timer.getSessionCount()).toBe(4);
    });
  });

  describe('Work Time Tracking', () => {
    test('should accurately track total work time across sessions', () => {
      timer.setWorkDuration(10 / 60); // 10 seconds
      timer.setShortBreakDuration(5 / 60); // 5 seconds

      // Complete 3 work sessions
      for (let i = 0; i < 3; i++) {
        timer.startWork();
        jest.advanceTimersByTime(10000);
        
        if (i < 2) {
          timer.startShortBreak();
          jest.advanceTimersByTime(5000);
        }
      }

      expect(timer.getTotalWorkTime()).toBe(30); // 3 sessions × 10 seconds
    });

    test('should only track time during work sessions', () => {
      timer.setWorkDuration(5 / 60);
      timer.setShortBreakDuration(5 / 60);

      // Work session
      timer.startWork();
      jest.advanceTimersByTime(3000);
      const workTimeAfterPartialWork = timer.getTotalWorkTime();
      expect(workTimeAfterPartialWork).toBe(3);

      // Break session
      timer.startShortBreak();
      jest.advanceTimersByTime(3000);
      const workTimeAfterBreak = timer.getTotalWorkTime();
      expect(workTimeAfterBreak).toBe(3); // Should not increase during break
    });
  });

  describe('Eye Check Reminders', () => {
    test('should emit eye check reminders at correct intervals', () => {
      timer.setWorkDuration(60 / 60); // 1 minute
      timer.setEyeCheckInterval(2.5 / 60); // 2.5 seconds

      timer.startWork();
      
      // First reminder after 2.5 seconds
      jest.advanceTimersByTime(2500);
      const eyeCheckEvents = events.filter(e => e.type === 'eyeCheckReminder');
      expect(eyeCheckEvents).toHaveLength(1);

      // Second reminder after another 2.5 seconds
      jest.advanceTimersByTime(2500);
      const eyeCheckEvents2 = events.filter(e => e.type === 'eyeCheckReminder');
      expect(eyeCheckEvents2).toHaveLength(2);
    });

    test('should not emit eye check during breaks', () => {
      timer.setShortBreakDuration(10 / 60);
      timer.setEyeCheckInterval(2 / 60);

      timer.startShortBreak();
      jest.advanceTimersByTime(10000);

      const eyeCheckEvents = events.filter(e => e.type === 'eyeCheckReminder');
      expect(eyeCheckEvents).toHaveLength(0);
    });
  });

  describe('Pause and Resume', () => {
    test('should maintain state when pausing and resuming', () => {
      timer.setWorkDuration(30 / 60); // 30 seconds

      timer.startWork();
      jest.advanceTimersByTime(10000); // 10 seconds

      // Pause
      timer.pause();
      const remainingAtPause = timer.getRemaining();
      const workTimeAtPause = timer.getTotalWorkTime();

      // Wait while paused
      jest.advanceTimersByTime(5000);

      // Resume
      timer.start();
      expect(timer.getRemaining()).toBe(remainingAtPause);
      expect(timer.getTotalWorkTime()).toBe(workTimeAtPause);

      // Continue and complete
      jest.advanceTimersByTime(20000);
      expect(timer.getTotalWorkTime()).toBe(30);
    });
  });

  describe('Settings Changes During Session', () => {
    test('should not affect current session when settings change', () => {
      timer.setWorkDuration(20 / 60);
      timer.startWork();

      expect(timer.getDuration()).toBe(20);

      // Change settings mid-session
      timer.setWorkDuration(30 / 60);
      
      // Current session should not be affected
      expect(timer.getDuration()).toBe(20);
      expect(timer.workDuration).toBe(30 * 60); // But setting is updated

      // Complete current session
      jest.advanceTimersByTime(20000);
      
      // Next session should use new duration
      timer.startWork();
      expect(timer.getDuration()).toBe(30 * 60);
    });
  });

  describe('Statistics Reset', () => {
    test('should reset all statistics correctly', () => {
      // Build up some statistics
      timer.setWorkDuration(5 / 60);
      
      for (let i = 0; i < 3; i++) {
        timer.startWork();
        jest.advanceTimersByTime(5000);
      }

      expect(timer.getSessionCount()).toBe(3);
      expect(timer.getTotalWorkTime()).toBe(15);

      // Reset
      timer.resetStats();

      expect(timer.getSessionCount()).toBe(0);
      expect(timer.getTotalWorkTime()).toBe(0);
      expect(events.some(e => e.type === 'statsReset')).toBe(true);
    });
  });

  describe('Event Ordering', () => {
    test('should emit events in correct order during session completion', () => {
      timer.setWorkDuration(1 / 60);
      timer.startWork();
      
      events = []; // Clear initial events
      jest.advanceTimersByTime(1000);

      // Check event order
      const eventTypes = events.map(e => e.type);
      const tickIndex = eventTypes.indexOf('tick');
      const finishedIndex = eventTypes.indexOf('finished');
      const workCompleteIndex = eventTypes.indexOf('workSessionComplete');
      const suggestBreakIndex = eventTypes.indexOf('suggestShortBreak');

      expect(tickIndex).toBeLessThan(finishedIndex);
      expect(finishedIndex).toBeLessThan(workCompleteIndex);
      expect(workCompleteIndex).toBeLessThan(suggestBreakIndex);
    });
  });

  describe('Real-world Scenarios', () => {
    test('should handle interrupted pomodoro with reset', () => {
      timer.setWorkDuration(25 / 60);
      timer.startWork();
      
      // Work for 10 seconds
      jest.advanceTimersByTime(10000);
      expect(timer.getTotalWorkTime()).toBe(10);
      
      // User resets (maybe distracted)
      timer.reset();
      expect(timer.getRemaining()).toBe(25 * 60);
      expect(timer.isRunning()).toBe(false);
      
      // Start again
      timer.startWork();
      jest.advanceTimersByTime(25000);
      
      // Total work time includes both sessions
      expect(timer.getTotalWorkTime()).toBe(35); // 10 + 25
      expect(timer.getSessionCount()).toBe(1); // Only one completed
    });

    test('should handle back-to-back sessions without breaks', () => {
      timer.setWorkDuration(10 / 60);
      
      // User skips breaks
      for (let i = 0; i < 3; i++) {
        timer.startWork();
        jest.advanceTimersByTime(10000);
        
        // Verify break suggestion but don't take it
        if (i === 2) {
          expect(events.some(e => e.type === 'suggestShortBreak')).toBe(true);
        }
      }
      
      expect(timer.getSessionCount()).toBe(3);
      expect(timer.getTotalWorkTime()).toBe(30);
    });
  });

  describe('Error Recovery', () => {
    test('should maintain consistency after rapid state changes', () => {
      timer.setWorkDuration(30 / 60);
      
      // Rapid start/stop/reset
      timer.startWork();
      jest.advanceTimersByTime(5000);
      timer.pause();
      timer.start();
      timer.reset();
      timer.startWork();
      timer.pause();
      timer.start();
      jest.advanceTimersByTime(25000);
      
      // Should complete normally despite chaos
      expect(timer.getRemaining()).toBe(0);
      expect(timer.getSessionCount()).toBe(1);
    });
  });
});