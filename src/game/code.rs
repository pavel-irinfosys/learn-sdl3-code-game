use std::fmt;
use std::fs;
use std::io;
use std::str::FromStr;

use crate::game::gird::DirectionEnum;

/// One box in the "Robot Code" row
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeInstruction {
    Move(DirectionEnum),
}

impl CodeInstruction {
    /// Movement as (dx, dy)
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

    /// Next state of a slot: empty -> UP -> RIGHT -> DOWN -> LEFT -> empty
    pub fn cycle(slot: Option<CodeInstruction>) -> Option<CodeInstruction> {
        use CodeInstruction::Move;
        match slot {
            None => Some(Move(DirectionEnum::UP)),
            Some(Move(DirectionEnum::UP)) => Some(Move(DirectionEnum::RIGHT)),
            Some(Move(DirectionEnum::RIGHT)) => Some(Move(DirectionEnum::DOWN)),
            Some(Move(DirectionEnum::DOWN)) => Some(Move(DirectionEnum::LEFT)),
            Some(Move(DirectionEnum::LEFT)) => None,
        }
    }

    /// Single letter used in the save file
    pub fn letter(self) -> char {
        match self {
            CodeInstruction::Move(DirectionEnum::UP) => 'U',
            CodeInstruction::Move(DirectionEnum::DOWN) => 'D',
            CodeInstruction::Move(DirectionEnum::LEFT) => 'L',
            CodeInstruction::Move(DirectionEnum::RIGHT) => 'R',
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

impl FromStr for CodeInstruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "U" | "UP" => Ok(CodeInstruction::Move(DirectionEnum::UP)),
            "D" | "DOWN" => Ok(CodeInstruction::Move(DirectionEnum::DOWN)),
            "L" | "LEFT" => Ok(CodeInstruction::Move(DirectionEnum::LEFT)),
            "R" | "RIGHT" => Ok(CodeInstruction::Move(DirectionEnum::RIGHT)),
            other => Err(format!("unknown instruction: {other}")),
        }
    }
}

pub const CODE_SLOTS: usize = 8;
pub type RobotCode = [Option<CodeInstruction>; CODE_SLOTS];

// ---------- storage ----------

/// RobotCode -> "R D R - - - - -"  ("-" = empty slot)
pub fn code_to_string(code: &RobotCode) -> String {
    code.iter()
        .map(|slot| match slot {
            Some(instr) => instr.letter().to_string(),
            None => "-".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// "R D R - - - - -" -> RobotCode (bad words become empty slots)
pub fn code_from_string(s: &str) -> RobotCode {
    let mut code: RobotCode = [None; CODE_SLOTS];
    for (i, word) in s.split_whitespace().take(CODE_SLOTS).enumerate() {
        if word != "-" {
            code[i] = word.parse::<CodeInstruction>().ok();
        }
    }
    code
}

pub fn save_code(path: &str, code: &RobotCode) -> io::Result<()> {
    fs::write(path, code_to_string(code))
}

/// Returns an empty code row if the file doesn't exist yet
pub fn load_code(path: &str) -> RobotCode {
    fs::read_to_string(path)
        .map(|s| code_from_string(&s))
        .unwrap_or([None; CODE_SLOTS])
}