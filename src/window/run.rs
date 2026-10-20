use super::{RunStatus, Window};
use crate::game::code::CODE_SLOTS;
use crate::game::gird::{Position, TileEnum};
use std::time::{Duration, Instant};

const STEP_DELAY: Duration = Duration::from_millis(500);

impl Window {
    pub(super) fn start_run(&mut self) {
        if self.run_index.is_some() {
            return; // already running
        }
        if let Some(start) = self.grid.find_start() {
            self.robot = start;
        }
        self.status = RunStatus::Running;
        self.run_index = Some(0);
        self.last_step = Instant::now();
    }

    pub(super) fn stop_run(&mut self) {
        self.run_index = None;
        self.status = RunStatus::Idle;
        if let Some(start) = self.grid.find_start() {
            self.robot = start;
        }
    }

    fn finish(&mut self, status: RunStatus) {
        self.run_index = None;
        self.status = status;
        match status {
            RunStatus::Won => println!("reached END!"),
            RunStatus::Blocked => println!("blocked by a rock or the edge"),
            RunStatus::OutOfMoves => println!("code ended before reaching END"),
            _ => {}
        }
    }

    /// Called every frame from render()
    pub(super) fn update(&mut self) {
        let Some(i) = self.run_index else { return };
        if self.last_step.elapsed() < STEP_DELAY {
            return;
        }
        self.last_step = Instant::now();

        if i >= CODE_SLOTS {
            self.finish(RunStatus::OutOfMoves);
            return;
        }
        self.run_index = Some(i + 1);

        let Some(instr) = self.code[i] else { return }; // empty slot: skip
        let (dx, dy) = instr.delta();
        let (nx, ny) = (self.robot.0 + dx, self.robot.1 + dy);
        let n = self.grid.size as i32;

        if nx < 0 || ny < 0 || nx >= n || ny >= n {
            self.finish(RunStatus::Blocked);
            return;
        }
        match self.grid.get(Position::new(nx as _, ny as _)) {
            TileEnum::ROCK => self.finish(RunStatus::Blocked),
            TileEnum::END => {
                self.robot = (nx, ny);
                self.finish(RunStatus::Won);
            }
            _ => self.robot = (nx, ny),
        }
    }
}
