use super::Window;
use crate::game::gird::{Position, TileEnum};
use crate::utility::ConvertColor;
use sdl3::pixels::Color;
use sdl3::render::FRect;

impl Window {
    pub fn render(&mut self) {
        self.clear("#1B1A1A".to_sdl_color());
        // draw call
        self.draw_grid();
        self.canvas.present();
    }

    fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_grid(&mut self) {
        let (w, h) = match self.canvas.output_size() {
            Ok(size) => size,
            Err(_) => return,
        };
        let n = self.grid.size as f32;
        let cell = (w.min(h) as f32 / n).floor(); // square cells that fit the window
        let offset_x = (w as f32 - cell * n) / 2.0; // center the board
        let offset_y = (h as f32 - cell * n) / 2.0;
        let gap = 2.0;

        for y in 0..self.grid.size {
            for x in 0..self.grid.size {
                let color = tile_color(self.grid.get(Position::new(x, y)));
                self.canvas.set_draw_color(color);
                let rect = FRect::new(
                    offset_x + x as f32 * cell + gap / 2.0,
                    offset_y + y as f32 * cell + gap / 2.0,
                    cell - gap,
                    cell - gap,
                );
                let _ = self.canvas.fill_rect(rect);
            }
        }
    }

}
fn tile_color(tile: TileEnum) -> Color {
    match tile {
        TileEnum::EMPTY => "#2E2D2D".to_sdl_color(),
        TileEnum::ROCK => "#7A7A7A".to_sdl_color(),
        TileEnum::START => "#3FB950".to_sdl_color(),
        TileEnum::END => "#E5534B".to_sdl_color(),
        TileEnum::UNKNOWN => "#B083F0".to_sdl_color(),
    }
}
