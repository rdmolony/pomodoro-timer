namespace PomodoroTimer {
    public class MainWindow : Adw.ApplicationWindow {
        public signal void eye_check_dialog_requested ();
        private Gtk.Label timer_label;
        private Gtk.Button start_pause_button;
        private Gtk.Button reset_button;
        private Gtk.ProgressBar progress_bar;
        private Gtk.Label session_label;
        private Gtk.Label next_20_20_20_label;
        private Gtk.Switch enable_20_20_20_switch;
        private Gtk.Button manual_eye_check_button;
        
        private Timer timer;
        private Timer twenty_twenty_timer;
        private int sessions_completed;
        private bool is_break_time;
        
        public MainWindow (Gtk.Application app) {
            Object (application: app);
            
            var settings = Application.settings;
            set_default_size (
                settings.get_int ("window-width"),
                settings.get_int ("window-height")
            );
            
            timer = new Timer ();
            twenty_twenty_timer = new Timer ();
            sessions_completed = 0;
            is_break_time = false;
            
            build_ui ();
            setup_signals ();
            reset_timer ();
            start_20_20_20_timer ();
            
            enable_20_20_20_switch.active = settings.get_boolean ("enable-20-20-20");
            enable_20_20_20_switch.notify["active"].connect (() => {
                settings.set_boolean ("enable-20-20-20", enable_20_20_20_switch.active);
                if (enable_20_20_20_switch.active) {
                    start_20_20_20_timer ();
                } else {
                    twenty_twenty_timer.stop ();
                }
            });
        }
        
        private void build_ui () {
            title = "Pomodoro Timer";
            
            var toast_overlay = new Adw.ToastOverlay ();
            set_content (toast_overlay);
            
            var main_box = new Gtk.Box (Gtk.Orientation.VERTICAL, 0);
            toast_overlay.set_child (main_box);
            
            var header_bar = new Adw.HeaderBar ();
            header_bar.set_title_widget (new Adw.WindowTitle ("Pomodoro Timer", null));
            main_box.append (header_bar);
            
            var menu_button = new Gtk.MenuButton ();
            menu_button.set_icon_name ("open-menu-symbolic");
            header_bar.pack_end (menu_button);
            
            var clamp = new Adw.Clamp ();
            clamp.set_maximum_size (400);
            clamp.set_tightening_threshold (300);
            main_box.append (clamp);
            
            var content_box = new Gtk.Box (Gtk.Orientation.VERTICAL, 24);
            content_box.set_margin_top (48);
            content_box.set_margin_bottom (48);
            content_box.set_margin_start (24);
            content_box.set_margin_end (24);
            clamp.set_child (content_box);
            
            session_label = new Gtk.Label ("Session 1");
            session_label.set_halign (Gtk.Align.CENTER);
            session_label.add_css_class ("title-2");
            content_box.append (session_label);
            
            var timer_box = new Gtk.Box (Gtk.Orientation.VERTICAL, 12);
            timer_box.set_halign (Gtk.Align.CENTER);
            content_box.append (timer_box);
            
            timer_label = new Gtk.Label ("25:00");
            timer_label.set_halign (Gtk.Align.CENTER);
            timer_label.add_css_class ("display");
            timer_box.append (timer_label);
            
            progress_bar = new Gtk.ProgressBar ();
            progress_bar.set_hexpand (true);
            progress_bar.set_fraction (0.0);
            timer_box.append (progress_bar);
            
            var button_box = new Gtk.Box (Gtk.Orientation.HORIZONTAL, 12);
            button_box.set_halign (Gtk.Align.CENTER);
            content_box.append (button_box);
            
            start_pause_button = new Gtk.Button.with_label ("Start");
            start_pause_button.set_size_request (120, -1);
            start_pause_button.add_css_class ("suggested-action");
            start_pause_button.add_css_class ("pill");
            button_box.append (start_pause_button);
            
            reset_button = new Gtk.Button.with_label ("Reset");
            reset_button.set_size_request (120, -1);
            reset_button.add_css_class ("pill");
            button_box.append (reset_button);
            
            var preferences_group = new Adw.PreferencesGroup ();
            preferences_group.set_title ("20-20-20 Rule");
            preferences_group.set_description ("Look at something 20 feet away for 20 seconds every 20 minutes");
            content_box.append (preferences_group);
            
            var switch_row = new Adw.ActionRow ();
            switch_row.set_title ("Enable 20-20-20 reminders");
            switch_row.set_subtitle ("Get notified every 20 minutes");
            preferences_group.add (switch_row);
            
            enable_20_20_20_switch = new Gtk.Switch ();
            enable_20_20_20_switch.set_valign (Gtk.Align.CENTER);
            switch_row.add_suffix (enable_20_20_20_switch);
            
            var status_row = new Adw.ActionRow ();
            status_row.set_title ("Status");
            preferences_group.add (status_row);
            
            next_20_20_20_label = new Gtk.Label ("Next reminder in 20:00");
            next_20_20_20_label.add_css_class ("monospace");
            status_row.add_suffix (next_20_20_20_label);
            
            var manual_trigger_row = new Adw.ActionRow ();
            manual_trigger_row.set_title ("Manual Trigger");
            manual_trigger_row.set_subtitle ("Test the eye check dialog");
            preferences_group.add (manual_trigger_row);
            
            manual_eye_check_button = new Gtk.Button.with_label ("Show Eye Check");
            manual_eye_check_button.set_valign (Gtk.Align.CENTER);
            manual_eye_check_button.add_css_class ("pill");
            manual_trigger_row.add_suffix (manual_eye_check_button);
        }
        
        private void setup_signals () {
            timer.tick.connect (on_timer_tick);
            timer.finished.connect (on_timer_finished);
            twenty_twenty_timer.finished.connect (on_20_20_20_reminder);
            
            start_pause_button.clicked.connect (on_start_pause_clicked);
            reset_button.clicked.connect (on_reset_clicked);
            manual_eye_check_button.clicked.connect (trigger_manual_eye_check);
            
            close_request.connect (() => {
                var settings = Application.settings;
                int width, height;
                get_default_size (out width, out height);
                settings.set_int ("window-width", width);
                settings.set_int ("window-height", height);
                return false;
            });
        }
        
        private void on_timer_tick (int remaining_seconds) {
            update_timer_display (remaining_seconds);
            update_progress_bar ();
        }
        
        private void on_timer_finished () {
            var app = (Application) application;
            var notification_manager = app.get_notification_manager ();
            
            if (is_break_time) {
                is_break_time = false;
                reset_timer ();
                session_label.label = "Session %d".printf(sessions_completed + 1);
                notification_manager.show_notification (
                    "Break finished!",
                    "Time to start your next Pomodoro session."
                );
            } else {
                sessions_completed++;
                is_break_time = true;
                
                bool is_long_break = sessions_completed % 4 == 0;
                int break_duration = is_long_break ? 
                    Application.settings.get_int ("long-break-duration") :
                    Application.settings.get_int ("break-duration");
                
                timer.set_duration (break_duration * 60);
                session_label.label = is_long_break ? "Long Break" : "Short Break";
                
                string break_type = is_long_break ? "long break" : "short break";
                notification_manager.show_notification (
                    "Pomodoro completed!",
                    "Great work! Time for a %s.".printf(break_type)
                );
            }
            
            start_pause_button.label = "Start";
        }
        
        private void on_20_20_20_reminder () {
            if (!Application.settings.get_boolean ("enable-20-20-20")) {
                return;
            }
            
            var app = (Application) application;
            var notification_manager = app.get_notification_manager ();
            
            notification_manager.show_20_20_20_notification ();
            start_20_20_20_timer ();
        }
        
        private void start_20_20_20_timer () {
            if (Application.settings.get_boolean ("enable-20-20-20")) {
                twenty_twenty_timer.set_duration (20 * 60);
                twenty_twenty_timer.start ();
                update_20_20_20_display ();
            }
        }
        
        private void update_20_20_20_display () {
            if (twenty_twenty_timer.is_running ()) {
                int remaining = twenty_twenty_timer.get_remaining_seconds ();
                int minutes = remaining / 60;
                int seconds = remaining % 60;
                next_20_20_20_label.label = "Next reminder in %d:%02d".printf(minutes, seconds);
            } else {
                next_20_20_20_label.label = "20-20-20 reminders disabled";
            }
        }
        
        private void on_start_pause_clicked () {
            if (timer.is_running ()) {
                timer.pause ();
                start_pause_button.label = "Resume";
            } else {
                timer.start ();
                start_pause_button.label = "Pause";
            }
        }
        
        private void on_reset_clicked () {
            reset_timer ();
            start_pause_button.label = "Start";
        }
        
        private void reset_timer () {
            timer.stop ();
            is_break_time = false;
            
            int duration = Application.settings.get_int ("pomodoro-duration") * 60;
            timer.set_duration (duration);
            
            update_timer_display (duration);
            progress_bar.fraction = 0.0;
            session_label.label = "Session %d".printf(sessions_completed + 1);
        }
        
        private void update_timer_display (int remaining_seconds) {
            int minutes = remaining_seconds / 60;
            int seconds = remaining_seconds % 60;
            timer_label.label = "%d:%02d".printf(minutes, seconds);
            
            if (twenty_twenty_timer.is_running ()) {
                update_20_20_20_display ();
            }
        }
        
        private void update_progress_bar () {
            double total = timer.get_total_duration ();
            double remaining = timer.get_remaining_seconds ();
            double progress = (total - remaining) / total;
            progress_bar.fraction = progress;
        }
        
        public bool has_manual_eye_check_button () {
            return manual_eye_check_button != null;
        }
        
        public void trigger_manual_eye_check () {
            eye_check_dialog_requested ();
            
            if (application is Application) {
                var app = (Application) application;
                var notification_manager = app.get_notification_manager ();
                notification_manager.set_main_window (this);
                
                var dialog = notification_manager.create_eye_check_dialog ();
                if (dialog != null) {
                    notification_manager.connect_eye_check_dialog_signals (dialog);
                    dialog.present ();
                }
            }
        }
    }
}