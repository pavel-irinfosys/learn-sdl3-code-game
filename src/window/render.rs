use super::Window;
use crate::game::gird::{Position, TileEnum};
use crate::utility::ConvertColor;
use sdl3::pixels::Color;
use sdl3::render::FRect;

const PANEL_H: f32 = 110.0; // height reserved for the code panel
use std::process;
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
        // process::exit(0);
    }

    fn draw_code_panel(&mut self) {
        let (w, h) = match self.canvas.output_size() {
            Ok(size) => size,
            Err(_) => return,
        };
        let (w, h) = (w as f32, h as f32);

        // panel background
        self.canvas.set_draw_color("#262626".to_sdl_color());
        let _ = self.canvas.fill_rect(FRect::new(0.0, h - PANEL_H, w, PANEL_H));

        // 8 boxes in one row
        let margin = 16.0;
        let gap = 8.0;
        let n = CODE_SLOTS as f32;
        let box_size = ((w - margin * 2.0 - gap * (n - 1.0)) / n).min(PANEL_H - margin * 2.0);
        let total_w = box_size * n + gap * (n - 1.0);
        let start_x = (w - total_w) / 2.0;
        let y = h - PANEL_H + (PANEL_H - box_size) / 2.0;

        for i in 0..CODE_SLOTS {
            let x = start_x + i as f32 * (box_size + gap);

            // box
            self.canvas.set_draw_color("#313647".to_sdl_color());
            let _ = self.canvas.fill_rect(FRect::new(x, y, box_size, box_size));
            self.canvas.set_draw_color("#A3B087".to_sdl_color());
            let _ = self.canvas.draw_rect(FRect::new(x, y, box_size, box_size));

            // arrow (empty slot = nothing drawn)
            if let Some(CodeInstruction::Move(dir)) = self.code[i] {
                self.draw_arrow(x + box_size / 2.0, y + box_size / 2.0, box_size * 0.3, dir);
            }
        }
    }

    fn draw_arrow(&mut self, cx: f32, cy: f32, size: f32, dir: DirectionEnum) {
        let (dx, dy) = match dir {
            DirectionEnum::UP => (0.0, -1.0),
            DirectionEnum::DOWN => (0.0, 1.0),
            DirectionEnum::LEFT => (-1.0, 0.0),
            DirectionEnum::RIGHT => (1.0, 0.0),
        };
        let (px, py) = (-dy, dx); // perpendicular
        let head = size * 0.6;

        let tail = FPoint::new(cx - dx * size, cy - dy * size);
        let tip = FPoint::new(cx + dx * size, cy + dy * size);
        let wing_a = FPoint::new(tip.x - dx * head + px * head, tip.y - dy * head + py * head);
        let wing_b = FPoint::new(tip.x - dx * head - px * head, tip.y - dy * head - py * head);

        self.canvas.set_draw_color("#FF7F11".to_sdl_color());
        // draw a few offset lines to make it thicker
        for o in [-1.0_f32, 0.0, 1.0] {
            let off = |p: FPoint| FPoint::new(p.x + px * o, p.y + py * o);
            let _ = self.canvas.draw_line(off(tail), off(tip));
            let _ = self.canvas.draw_line(off(tip), off(wing_a));
            let _ = self.canvas.draw_line(off(tip), off(wing_b));
        }
    }
}
fn tile_color(tile: TileEnum) -> Color {
    match tile {
        TileEnum::EMPTY => "#A3B087".to_sdl_color(),
        TileEnum::ROCK => "#435663".to_sdl_color(),
        TileEnum::START => "#BD4444".to_sdl_color(),
        TileEnum::END => "#FF7F11".to_sdl_color(),
        TileEnum::UNKNOWN => "#313647".to_sdl_color(),
    }
}
