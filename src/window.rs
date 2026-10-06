
use sdl3::render::Canvas;
use sdl3::{ EventPump, Sdl };

mod view;
mod window;

pub struct Window {
    _sdl: Sdl,
    canvas: Canvas<sdl3::video::Window>,
    event_pump: EventPump,
}
