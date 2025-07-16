namespace PomodoroTimer {
    public class EyeCheckDialog : Gtk.Window {
        public signal void dismissed ();
        public signal void snoozed ();
        
        private Gtk.Label title_label;
        private Gtk.DrawingArea eye_drawing;
        private Gtk.Label instructions_label;
        private Gtk.Button dismiss_button;
        private Gtk.Button snooze_button;
        
        public EyeCheckDialog (Gtk.Window parent) {
            set_transient_for (parent);
            set_modal (true);
            fullscreen ();
            build_ui ();
        }
        
        private void build_ui () {
            var overlay = new Gtk.Overlay ();
            set_child (overlay);
            
            var content_box = new Gtk.Box (Gtk.Orientation.VERTICAL, 32);
            content_box.set_halign (Gtk.Align.CENTER);
            content_box.set_valign (Gtk.Align.CENTER);
            overlay.set_child (content_box);
            
            title_label = new Gtk.Label ("20-20-20 Eye Rest");
            title_label.set_markup ("<span size='xx-large' weight='bold'>20-20-20 Eye Rest</span>");
            content_box.append (title_label);
            
            eye_drawing = new Gtk.DrawingArea ();
            eye_drawing.set_size_request (200, 120);
            content_box.append (eye_drawing);
            
            instructions_label = new Gtk.Label (null);
            instructions_label.set_markup ("<span size='large'>Look at something 20 feet away\nfor 20 seconds to rest your eyes</span>");
            instructions_label.set_justify (Gtk.Justification.CENTER);
            content_box.append (instructions_label);
            
            var button_box = new Gtk.Box (Gtk.Orientation.HORIZONTAL, 16);
            button_box.set_halign (Gtk.Align.CENTER);
            content_box.append (button_box);
            
            dismiss_button = new Gtk.Button.with_label ("Got it!");
            dismiss_button.set_size_request (140, 48);
            dismiss_button.add_css_class ("suggested-action");
            dismiss_button.add_css_class ("pill");
            button_box.append (dismiss_button);
            
            snooze_button = new Gtk.Button.with_label ("Remind me in 5 minutes");
            snooze_button.set_size_request (200, 48);
            snooze_button.add_css_class ("pill");
            button_box.append (snooze_button);
        }
        
        public void simulate_dismiss_click () {
            dismissed ();
        }
        
        public void simulate_snooze_click () {
            snoozed ();
        }
        
        public void simulate_esc_key () {
            dismissed ();
        }
        
        public bool has_title_label () {
            return title_label != null;
        }
        
        public bool has_eye_drawing () {
            return eye_drawing != null;
        }
        
        public bool has_instructions_label () {
            return instructions_label != null;
        }
        
        public bool has_dismiss_button () {
            return dismiss_button != null;
        }
        
        public bool has_snooze_button () {
            return snooze_button != null;
        }
    }
}