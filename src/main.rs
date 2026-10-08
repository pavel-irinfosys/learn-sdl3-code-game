mod window;
mod utility;
mod game;

use window::Window;
use crate::game::gird::{Grid, Position, TileEnum};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let mut grid = Grid::new(8);
    grid.set(Position::new(2, 3), TileEnum::START);
    grid.set(Position::new(6, 3), TileEnum::END);
    grid.set_random_rocks(32, seed);
    grid.print();

    // let mut win = Window::new("My App", 800, 640)?;
    // win.view();
    Ok(())
}
