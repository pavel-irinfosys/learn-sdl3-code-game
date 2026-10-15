use crate::game::code::RobotCode;
use crate::game::gird::Grid;
use sdl3::render::Canvas;
use sdl3::{EventPump, Sdl};

mod events;
mod render;
mod view;
mod window;

pub struct Window {
    _sdl: Sdl,
    canvas: Canvas<sdl3::video::Window>,
    event_pump: EventPump,
    grid: Grid,
    code: RobotCode,
    pub reset_pressed: bool,
    icon_pixels: Vec<u8>,
    icon_w: u32,
    icon_h: u32,
}
