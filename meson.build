project('pomodoro-timer', 'vala', 'c',
  version: '1.0.0',
  license: 'GPL-3.0-or-later',
  meson_version: '>= 0.59.0',
  default_options: [
    'warning_level=2',
    'werror=false',
  ]
)

gnome = import('gnome')
i18n = import('i18n')

glib_dep = dependency('glib-2.0', version: '>= 2.72')
gobject_dep = dependency('gobject-2.0', version: '>= 2.72')
gtk_dep = dependency('gtk4', version: '>= 4.8')
adwaita_dep = dependency('libadwaita-1', version: '>= 1.2')
notify_dep = dependency('libnotify', version: '>= 0.7')
gsound_dep = dependency('gsound', version: '>= 1.0')

app_id = 'com.github.user.PomodoroTimer'
app_name = 'Pomodoro Timer'

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()

subdir('src')
subdir('data')
subdir('tests')

gnome.post_install(
  glib_compile_schemas: true,
  gtk_update_icon_cache: false,
  update_desktop_database: true,
)