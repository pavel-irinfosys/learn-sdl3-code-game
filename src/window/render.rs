use super::Window;
use crate::game::code::{CODE_SLOTS, CodeInstruction};
use crate::game::gird::{DirectionEnum, Position, TileEnum};
use crate::utility::ConvertColor;
use crate::window::RunStatus;
use sdl3::pixels::Color;
use sdl3::render::{FPoint, FRect};
use std::f32::consts::PI;

const BTN_SIZE: f32 = 44.0;
const BTN_MARGIN: f32 = 16.0;

const PANEL_H: f32 = 110.0; // height reserved for the code panel

impl Window {
    /// Button position and size as (x, y, size)
    pub(super) fn reset_button(&self) -> (f32, f32, f32) {
        let w = self
            .canvas
            .output_size()
            .map(|(w, _)| w as f32)
            .unwrap_or(800.0);
        (w - BTN_MARGIN - BTN_SIZE, BTN_MARGIN, BTN_SIZE)
    }

    pub(super) fn hit_reset_button(&self, mx: f32, my: f32) -> bool {
        let (x, y, s) = self.reset_button();
        mx >= x && mx <= x + s && my >= y && my <= y + s
    }

    /// Box position and size for code slot i, as (x, y, size)
    pub(super) fn code_box(&self, i: usize) -> (f32, f32, f32) {
        let (w, h) = self
            .canvas
            .output_size()
            .map(|(w, h)| (w as f32, h as f32))
            .unwrap_or((800.0, 640.0));

        let margin = 16.0;
        let gap = 8.0;
        let n = CODE_SLOTS as f32;
        let size = ((w - margin * 2.0 - gap * (n - 1.0)) / n).min(PANEL_H - margin * 2.0);
        let total_w = size * n + gap * (n - 1.0);
        let start_x = (w - total_w) / 2.0;
        let y = h - PANEL_H + (PANEL_H - size) / 2.0;

        (start_x + i as f32 * (size + gap), y, size)
    }

    /// Which code slot is under the mouse, if any
    pub(super) fn hit_code_box(&self, mx: f32, my: f32) -> Option<usize> {
        (0..CODE_SLOTS).find(|&i| {
            let (x, y, s) = self.code_box(i);
            mx >= x && mx <= x + s && my >= y && my <= y + s
        })
    }

    pub fn render(&mut self) {
        self.update(); // advance the run
        self.clear("#1B1A1A".to_sdl_color());
        self.draw_grid();
        self.draw_robot();
        self.draw_code_panel();
        self.draw_reset_button();
        self.draw_run_button();
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

        let padding = 16.0;
        let area_w = w as f32 - padding * 2.0;
        let area_h = h as f32 - PANEL_H - padding * 2.0;

        let n = self.grid.size as f32;
        let cell = (area_w.min(area_h) / n).floor();
        let offset_x = (w as f32 - cell * n) / 2.0;
        let offset_y = padding + (area_h - cell * n) / 2.0;
        let gap = 4.0;

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

    fn draw_code_panel(&mut self) {
        let (w, h) = match self.canvas.output_size() {
            Ok(size) => size,
            Err(_) => return,
        };
        let (w, h) = (w as f32, h as f32);

        // panel background
        self.canvas.set_draw_color("#262626".to_sdl_color());
        let _ = self
            .canvas
            .fill_rect(FRect::new(0.0, h - PANEL_H, w, PANEL_H));

        for i in 0..CODE_SLOTS {
            let (x, y, s) = self.code_box(i);

            self.canvas.set_draw_color("#313647".to_sdl_color());
            let _ = self.canvas.fill_rect(FRect::new(x, y, s, s));
            self.canvas.set_draw_color("#A3B087".to_sdl_color());
            let _ = self.canvas.draw_rect(FRect::new(x, y, s, s));

            // arrow (empty slot = nothing drawn)
            if let Some(CodeInstruction::Move(dir)) = self.code[i] {
                self.draw_arrow(x + s / 2.0, y + s / 2.0, s * 0.3, dir);
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
        let (px, py) = (-dy, dx);
        let head = size * 0.6;

        let tail = FPoint::new(cx - dx * size, cy - dy * size);
        let tip = FPoint::new(cx + dx * size, cy + dy * size);
        let wing_a = FPoint::new(tip.x - dx * head + px * head, tip.y - dy * head + py * head);
        let wing_b = FPoint::new(tip.x - dx * head - px * head, tip.y - dy * head - py * head);

        self.canvas.set_draw_color("#FF7F11".to_sdl_color());
        for o in [-1.0_f32, 0.0, 1.0] {
            let off = |p: FPoint| FPoint::new(p.x + px * o, p.y + py * o);
            let _ = self.canvas.draw_line(off(tail), off(tip));
            let _ = self.canvas.draw_line(off(tip), off(wing_a));
            let _ = self.canvas.draw_line(off(tip), off(wing_b));
        }
    }

    fn draw_reset_button(&mut self) {
        let (x, y, s) = self.reset_button();

        let bg = if self.reset_pressed {
            "#1F2330"
        } else {
            "#313647"
        };
        self.canvas.set_draw_color(bg.to_sdl_color());
        let _ = self.canvas.fill_rect(FRect::new(x, y, s, s));
        self.canvas.set_draw_color("#A3B087".to_sdl_color());
        let _ = self.canvas.draw_rect(FRect::new(x, y, s, s));

        // circular arrow symbol
        let nudge = if self.reset_pressed { 2.0 } else { 0.0 };
        let (cx, cy) = (x + s / 2.0 + nudge, y + s / 2.0 + nudge);
        let r = s * 0.28;
        let (start, end) = (PI * 0.25, PI * 1.85); // open circle
        let steps = 24;
        let point = |a: f32| FPoint::new(cx + r * a.cos(), cy + r * a.sin());

        self.canvas.set_draw_color("#FF7F11".to_sdl_color());
        for i in 0..steps {
            let a1 = start + (end - start) * i as f32 / steps as f32;
            let a2 = start + (end - start) * (i + 1) as f32 / steps as f32;
            // two lines side by side make the stroke thicker
            for o in [0.0_f32, 1.0] {
                let _ = self.canvas.draw_line(
                    FPoint::new(point(a1).x, point(a1).y + o),
                    FPoint::new(point(a2).x, point(a2).y + o),
                );
            }
        }

        // arrow head at the end of the arc
        let tip = point(end);
        let tangent = (-end.sin(), end.cos());
        let normal = (end.cos(), end.sin());
        let head = r * 0.6;
        for sign in [1.0_f32, -1.0] {
            let _ = self.canvas.draw_line(
                tip,
                FPoint::new(
                    tip.x - tangent.0 * head + normal.0 * head * sign,
                    tip.y - tangent.1 * head + normal.1 * head * sign,
                ),
            );
        }
    }

    /// Run button (top-left): (x, y, size)
    pub(super) fn run_button(&self) -> (f32, f32, f32) {
        (BTN_MARGIN, BTN_MARGIN, BTN_SIZE)
    }

    pub(super) fn hit_run_button(&self, mx: f32, my: f32) -> bool {
        let (x, y, s) = self.run_button();
        mx >= x && mx <= x + s && my >= y && my <= y + s
    }

    /// Grid layout as (offset_x, offset_y, cell)
    fn grid_layout(&self) -> (f32, f32, f32) {
        let (w, h) = self.canvas.output_size().unwrap_or((800, 640));
        let padding = 16.0;
        let area_w = w as f32 - padding * 2.0;
        let area_h = h as f32 - PANEL_H - padding * 2.0;
        let n = self.grid.size as f32;
        let cell = (area_w.min(area_h) / n).floor();
        (
            (w as f32 - cell * n) / 2.0,
            padding + (area_h - cell * n) / 2.0,
            cell,
        )
    }

    fn draw_robot(&mut self) {
        let (ox, oy, cell) = self.grid_layout();
        let color = match self.status {
            RunStatus::Won => "#6BCB77",
            RunStatus::Blocked | RunStatus::OutOfMoves => "#E63946",
            _ => "#F1FAEE",
        };
        let pad = cell * 0.25;
        self.canvas.set_draw_color(color.to_sdl_color());
        let _ = self.canvas.fill_rect(FRect::new(
            ox + self.robot.0 as f32 * cell + pad,
            oy + self.robot.1 as f32 * cell + pad,
            cell - pad * 2.0,
            cell - pad * 2.0,
        ));
    }

    fn draw_run_button(&mut self) {
        let (x, y, s) = self.run_button();
        let bg = if self.status == RunStatus::Running {
            "#1F2330"
        } else {
            "#313647"
        };
        self.canvas.set_draw_color(bg.to_sdl_color());
        let _ = self.canvas.fill_rect(FRect::new(x, y, s, s));
        self.canvas.set_draw_color("#A3B087".to_sdl_color());
        let _ = self.canvas.draw_rect(FRect::new(x, y, s, s));

        // play triangle, filled with vertical lines
        let (x0, x1, cy) = (x + s * 0.35, x + s * 0.75, y + s / 2.0);
        let mut px = x0;
        self.canvas.set_draw_color("#6BCB77".to_sdl_color());
        while px <= x1 {
            let half = (x1 - px) / (x1 - x0) * s * 0.25;
            let _ = self
                .canvas
                .draw_line(FPoint::new(px, cy - half), FPoint::new(px, cy + half));
            px += 1.0;
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
