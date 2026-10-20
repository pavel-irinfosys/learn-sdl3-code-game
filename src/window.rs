use crate::game::code::RobotCode;
use crate::game::gird::Grid;
use sdl3::render::Canvas;
use sdl3::{EventPump, Sdl};
use std::time::Instant;

mod events;
mod render;
mod run; // new
mod view;
mod window;

#[derive(Clone, Copy, PartialEq)]
pub enum RunStatus {
    Idle,
    Running,
    Won,
    Blocked,
    OutOfMoves,
}

pub struct Window {
    _sdl: Sdl,
    canvas: Canvas<sdl3::video::Window>,
    event_pump: EventPump,
    grid: Grid,
    pub code: RobotCode,
    pub reset_pressed: bool,
    icon_pixels: Vec<u8>,
    icon_w: u32,
    icon_h: u32,
    robot: (i32, i32),
    run_index: Option<usize>,
    last_step: Instant,
    status: RunStatus,
}
