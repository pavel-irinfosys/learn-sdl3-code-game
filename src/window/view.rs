
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use std::time::Duration;

use super::Window;

impl Window {
    pub fn view(&mut self) {
        'running: loop {
            // 1. handle events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                    _ => {}
                }
            }

            // 2. draw
            self.canvas.set_draw_color(Color::RGB(30, 30, 60));
            self.canvas.clear();
            self.canvas.present();

            // 3. don't burn the CPU
            std::thread::sleep(Duration::from_millis(16));
        }
    }
}
