mod game;
mod utility;
mod window;

use crate::game::code::load_code;
use crate::game::gird::Grid;
use std::time::{SystemTime, UNIX_EPOCH};
use window::Window;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let mut grid = Grid::new(4);
    grid.set_random_rocks(3, seed);
    grid.set_random_start_end(seed);
    grid.print();

    let mut win = Window::new("My App", 800, 640, grid)?;
    win.code = load_code("robot_code.txt");
    win.view();
    Ok(())
}