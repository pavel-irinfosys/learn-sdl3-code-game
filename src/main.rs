mod game;
mod utility;
mod window;

use crate::game::gird::{Grid, Position, TileEnum};
use std::time::{SystemTime, UNIX_EPOCH};
use window::Window;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let mut grid = Grid::new(4);
    grid.set_random_rocks(3, seed);
    grid.set(Position::new(2, 2), TileEnum::START);
    grid.set(Position::new(0, 2), TileEnum::END);
    grid.print();

    let mut win = Window::new("My App", 800, 640, grid)?;
    win.view();
    Ok(())
}
