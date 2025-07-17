# Full-Screen Eye Check Alert Implementation Plan

## Tests to Implement (in order)

### EyeCheckDialog Tests
- [x] EyeCheckDialog should be created with parent window
- [x] EyeCheckDialog should be modal and fullscreen
- [x] EyeCheckDialog should emit dismissed signal when Got it button clicked
- [x] EyeCheckDialog should emit snoozed signal when snooze button clicked
- [x] EyeCheckDialog should close when ESC key pressed
- [x] EyeCheckDialog should have proper CSS styling applied

### NotificationManager Integration Tests
- [x] NotificationManager should show full-screen eye check dialog instead of system notification
- [x] NotificationManager should handle eye check dialog dismissed signal
- [x] NotificationManager should handle eye check dialog snoozed signal and schedule reminder
- [x] NotificationManager should pass main window reference to eye check dialog

### Window Integration Tests
- [x] MainWindow should provide reference to NotificationManager for eye check dialog
- [x] Eye check dialog should appear on top of main window when 20-20-20 timer triggers
- [x] MainWindow should have manual trigger button for eye check dialog
- [x] Manual trigger button should show eye check dialog when clicked

## Implementation Notes
- Replace system notification with full-screen modal dialog
- Use Cairo for drawing eye SVG
- Implement proper signal handling for dismiss/snooze actions
- Ensure dialog is properly modal and covers entire screen