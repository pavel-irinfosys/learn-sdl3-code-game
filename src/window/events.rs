use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::mouse::MouseButton;
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
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    if self.hit_reset_button(x, y) {
                        self.reset_pressed = true;
                    }
                }
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, x, y, .. } => {
                    if self.reset_pressed && self.hit_reset_button(x, y) {
                        self.reset();
                    }
                    self.reset_pressed = false;
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