


use super::Window;
use std::time::Duration;

impl Window {
    pub fn view(&mut self) {
        'main_loop: loop {
            // 1. handle events
            if !self.handle_event() {
                break 'main_loop;
            }

            // 2. render
            self.render();

            // 3. don't burn the CPU
            std::thread::sleep(Duration::from_millis(16));
        }
    }
}
