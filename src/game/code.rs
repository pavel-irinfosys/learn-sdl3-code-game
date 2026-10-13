use std::fmt;

use crate::game::gird::DirectionEnum;

/// One box in the "Robot Code" row
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeInstruction {
    Move(DirectionEnum),
}

impl CodeInstruction {
    pub fn delta(self) -> (i32, i32) {
        match self {
            CodeInstruction::Move(dir) => match dir {
                DirectionEnum::UP => (0, -1),
                DirectionEnum::LEFT => (-1, 0),
                DirectionEnum::DOWN => (0, 1),
                DirectionEnum::RIGHT => (1, 0),
            },
        }
    }
}

impl fmt::Display for CodeInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodeInstruction::Move(DirectionEnum::UP) => write!(f, "↑"),
            CodeInstruction::Move(DirectionEnum::DOWN) => write!(f, "↓"),
            CodeInstruction::Move(DirectionEnum::LEFT) => write!(f, "←"),
            CodeInstruction::Move(DirectionEnum::RIGHT) => write!(f, "→"),
        }
    }
}
