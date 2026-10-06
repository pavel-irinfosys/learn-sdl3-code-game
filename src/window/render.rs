use crate::utility::ConvertColor;
use super::Window;

impl Window {
    pub fn render(&mut self) {
        self.canvas.set_draw_color("#1B1A1A".to_sdl_color());
        self.canvas.clear();
        self.canvas.present();
    }
}
