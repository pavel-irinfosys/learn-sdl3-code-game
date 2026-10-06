use sdl3::event::Event;
use sdl3::keyboard::Keycode;

use super::Window;

impl Window {
    pub fn handle_event(&mut self) -> bool{
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return false,
                _ => {}
            }
        }
        return true
    }
}