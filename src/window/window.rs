type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

use super::Window;

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> AppResult<Self> {
        let sdl = sdl3::init()?;
        let video = sdl.video()?;

        let sdl_window = video
            .window(title, width, height)
            .position_centered()
            .resizable()
            .build()?;

        let canvas = sdl_window.into_canvas();
        let event_pump = sdl.event_pump()?;

        Ok(Self { _sdl: sdl, canvas, event_pump })
    }
}
