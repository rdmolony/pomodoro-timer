# Copy source files for testing (workaround for meson cross-directory vala limitation)
timer_copy = custom_target('copy-timer',
  input: '../src/timer.vala',
  output: 'timer.vala',
  command: ['cp', '@INPUT@', '@OUTPUT@'],
  build_by_default: false
)

notification_manager_copy = custom_target('copy-notification-manager',
  input: '../src/notification-manager.vala',
  output: 'notification-manager.vala',
  command: ['cp', '@INPUT@', '@OUTPUT@'],
  build_by_default: false
)

application_copy = custom_target('copy-application',
  input: '../src/application.vala',
  output: 'application.vala',
  command: ['cp', '@INPUT@', '@OUTPUT@'],
  build_by_default: false
)

window_copy = custom_target('copy-window',
  input: '../src/window.vala',
  output: 'window.vala',
  command: ['cp', '@INPUT@', '@OUTPUT@'],
  build_by_default: false
)

# Timer tests
timer_test_exe = executable(
  'timer-tests',
  files('test-timer.vala') + [timer_copy],
  dependencies: [glib_dep, gobject_dep],
  install: false,
  build_by_default: false,
  vala_args: ['--target-glib=2.72']
)

# NotificationManager tests
notification_manager_test_exe = executable(
  'notification-manager-tests',
  files('test-notification-manager.vala') + [notification_manager_copy, application_copy, window_copy, timer_copy],
  dependencies: [glib_dep, gobject_dep, gtk_dep, adwaita_dep, notify_dep, gsound_dep],
  install: false,
  build_by_default: false,
  vala_args: ['--target-glib=2.72']
)

# Application tests
application_test_exe = executable(
  'application-tests',
  files('test-application.vala') + [application_copy, window_copy, timer_copy, notification_manager_copy],
  dependencies: [glib_dep, gobject_dep, gtk_dep, adwaita_dep, notify_dep, gsound_dep],
  install: false,
  build_by_default: false,
  vala_args: ['--target-glib=2.72']
)

test('timer-tests', timer_test_exe)
test('notification-manager-tests', notification_manager_test_exe)
test('application-tests', application_test_exe)