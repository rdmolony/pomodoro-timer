namespace PomodoroTimer {
    public class Timer : Object {
        public signal void tick (int remaining_seconds);
        public signal void finished ();
        
        private int total_duration;
        private int remaining_seconds;
        private uint timer_id;
        private bool running;
        
        public Timer () {
            total_duration = 0;
            remaining_seconds = 0;
            timer_id = 0;
            running = false;
        }
        
        public void set_duration (int seconds) {
            total_duration = seconds;
            remaining_seconds = seconds;
        }
        
        public int get_total_duration () {
            return total_duration;
        }
        
        public int get_remaining_seconds () {
            return remaining_seconds;
        }
        
        public bool is_running () {
            return running;
        }
        
        public void start () {
            if (running) return;
            
            running = true;
            timer_id = Timeout.add (1000, on_timer_tick);
        }
        
        public void pause () {
            if (!running) return;
            
            running = false;
            Source.remove (timer_id);
            timer_id = 0;
        }
        
        public void stop () {
            if (running) {
                Source.remove (timer_id);
                timer_id = 0;
            }
            running = false;
        }
        
        private bool on_timer_tick () {
            remaining_seconds--;
            tick (remaining_seconds);
            
            if (remaining_seconds <= 0) {
                running = false;
                timer_id = 0;
                finished ();
                return false;
            }
            
            return true;
        }
    }
}