sources = files(
  'main.vala',
  'application.vala',
  'window.vala',
  'timer.vala',
  'notification-manager.vala'
)

resources = gnome.compile_resources('pomodoro-timer-resources',
  '../data/com.github.user.PomodoroTimer.gresource.xml',
  source_dir: '../data',
  c_name: 'pomodoro_timer'
)

dependencies = [
  glib_dep,
  gobject_dep,
  gtk_dep,
  adwaita_dep,
  notify_dep,
  gsound_dep,
]

executable('pomodoro-timer',
  sources,
  resources,
  dependencies: dependencies,
  install: true,
  install_dir: bindir,
  vala_args: [
    '--pkg=gtk4',
    '--pkg=libadwaita-1',
    '--pkg=libnotify',
    '--pkg=gsound',
    '--target-glib=2.72'
  ]
)