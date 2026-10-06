use sdl3::pixels::Color;
use crate::utility::ConvertColor;
use super::Window;

impl Window {
    pub fn render(&mut self) {
        self.clear("#1B1A1A".to_sdl_color());
        // draw call
        self.canvas.present();
    }

    fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }
}
