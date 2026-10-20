use super::Window;
use crate::game::code::{CodeInstruction, load_code, save_code};
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::mouse::MouseButton;

const SAVE_PATH: &str = "robot_code.txt";

impl Window {
    pub fn handle_event(&mut self) -> bool {
        let events: Vec<Event> = self.event_pump.poll_iter().collect();
        for event in events {
            match event {
                Event::Quit { .. } => return false,
                Event::KeyUp { .. } | Event::KeyDown { .. } => {
                    if !self.handle_keyboard(&event) {
                        return false;
                    }
                }
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    if mouse_btn == MouseButton::Left && self.hit_reset_button(x, y) {
                        self.reset_pressed = true;
                    } else if let Some(i) = self.hit_code_box(x, y) {
                        match mouse_btn {
                            MouseButton::Left => {
                                self.code[i] = CodeInstruction::cycle(self.code[i]);
                                self.save();
                            }
                            MouseButton::Right => {
                                self.code[i] = None;
                                self.save();
                            }
                            _ => {}
                        }
                    }
                }
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, x, y, .. } => {
                    if self.reset_pressed && self.hit_reset_button(x, y) {
                        self.reset();
                        self.save();
                    }
                    self.reset_pressed = false;
                }
                _ => {}
            }
        }
        true
    }

    fn save(&self) {
        if let Err(e) = save_code(SAVE_PATH, &self.code) {
            eprintln!("save failed: {e}");
        }
    }

    fn handle_keyboard(&mut self, event: &Event) -> bool {
        match event {
            Event::KeyDown { keycode: Some(key), repeat, .. } => {
                if *key == Keycode::Escape {
                    return false;
                }
                if !*repeat {
                    println!("key down: {:?}", key);
                    match *key {
                        Keycode::R => {
                            self.reset();
                            self.save();
                        }
                        Keycode::S => {
                            self.save();
                            println!("code saved");
                        }
                        Keycode::L => {
                            self.code = load_code(SAVE_PATH);
                            println!("code loaded");
                        }
                        _ => {}
                    }
                }
            }
            Event::KeyUp { keycode: Some(key), .. } => {
                println!("key up: {:?}", key);
            }
            _ => {}
        }
        true
    }
}