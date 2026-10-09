use sdl3::event::Event;
use sdl3::keyboard::Keycode;

use super::Window;

impl Window {
    pub fn handle_event(&mut self) -> bool{
        let events: Vec<Event> = self.event_pump.poll_iter().collect();
        for event in events {
            match event {
                Event::Quit { .. } => return false,
                Event::KeyUp { .. } | Event::KeyDown { .. } => {
                    if !self.handle_keyboard(&event) {
                        return false;
                    }
                }
                _ => {}
            }
        }
        true
    }

    fn handle_keyboard(&mut self, event: &Event) -> bool {
        match event {
            Event::KeyDown { keycode: Some(key), repeat, ..} => {
                if *key == Keycode::Escape {
                    return false;
                }
                if !*repeat {
                    println!("key down: {:?}", key);
                }
            }
            Event::KeyUp { keycode: Some(key), ..} => {
                println!("key up: {:?}", key);
            }
            _ => {

            }
        }
        true
    }
}