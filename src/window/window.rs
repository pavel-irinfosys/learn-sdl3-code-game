type AppResult<T> = Result<T, Box<dyn std::error::Error>>;
use crate::game::code::{CODE_SLOTS};
use crate::game::gird::Grid;
use super::Window;

impl Window {
    pub fn new(title: &str, width: u32, height: u32, grid: Grid) -> AppResult<Self> {
        let sdl = sdl3::init()?;
        let video = sdl.video()?;

        let sdl_window = video
            .window(title, width, height)
            .position_centered()
            .resizable()
            .build()?;

        let canvas = sdl_window.into_canvas();
        let event_pump = sdl.event_pump()?;
        // inside Window::new, before Ok(Self { ... })
        let ttf = Box::leak(Box::new(sdl3::ttf::init()?)); // leaked so the font can be 'static

        let emoji_font = ttf.load_font("assets/NotoColorEmoji-Regular.ttf", 32.0)?;

        Ok(Self { _sdl: sdl, canvas, event_pump, grid, code: [None; CODE_SLOTS], emoji_font })
    }
}
