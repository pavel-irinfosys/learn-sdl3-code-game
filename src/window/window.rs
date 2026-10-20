type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

use super::{RunStatus, Window};
use crate::game::code::CODE_SLOTS;
use crate::game::gird::Grid;
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

        // must be computed before `grid` is moved into Self
        let robot = grid.find_start().unwrap_or((0, 0));
        let sound = super::Sound::new(&sdl).ok(); // before `_sdl: sdl` moves sdl
        Ok(Self {
            _sdl: sdl,
            canvas,
            event_pump,
            grid,
            code: [None; CODE_SLOTS],
            reset_pressed: false,
            robot,
            run_index: None,
            last_step: Instant::now(),
            status: RunStatus::Idle,
            sound,
        })
    }

    pub fn reset(&mut self) {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        self.grid.reset(3, seed);
        self.code = [None; CODE_SLOTS];
        self.stop_run();
    }
}
