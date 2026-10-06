use sdl3::pixels::Color;

use super::Window;

impl Window {
    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }
}
