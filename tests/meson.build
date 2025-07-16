# Copy timer source for testing (workaround for meson cross-directory vala limitation)
timer_copy = custom_target('copy-timer',
  input: '../src/timer.vala',
  output: 'timer.vala',
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

test('timer-tests', timer_test_exe)