type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

use super::{RunStatus, Window};
use crate::game::code::CODE_SLOTS;
use crate::game::gird::{Grid, Position};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

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

        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/reset.png");
        let img = image::open(path)?.to_rgba8();
        let (icon_w, icon_h) = img.dimensions();
        let icon_pixels = img.into_raw();

        // must be computed before `grid` is moved into Self
        let robot = grid.find_start().unwrap_or((0, 0));

        Ok(Self {
            _sdl: sdl,
            canvas,
            event_pump,
            grid,
            code: [None; CODE_SLOTS],
            reset_pressed: false,
            icon_pixels,
            icon_w,
            icon_h,
            robot,
            run_index: None,
            last_step: Instant::now(),
            status: RunStatus::Idle,
        })
    }

    pub fn reset(&mut self) {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        self.grid
            .reset(3, seed, Position::new(2, 2), Position::new(0, 2));
        self.code = [None; CODE_SLOTS];
        self.stop_run();
    }
}
