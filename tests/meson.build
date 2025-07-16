# Timer tests
timer_test_exe = executable(
  'timer-tests',
  files('test-timer.vala', 'timer.vala'),
  dependencies: [glib_dep, gobject_dep],
  install: false,
  build_by_default: false
)

test('timer-tests', timer_test_exe)