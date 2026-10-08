#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileEnum {
    EMPTY,
    START,
    END,
    ROCK,
    UNKNOWN,
}

impl TileEnum {
    pub fn symbol(self) -> char {
        match self {
            TileEnum::EMPTY => '.',
            TileEnum::ROCK => '#',
            TileEnum::START => 'S',
            TileEnum::END => 'E',
            TileEnum::UNKNOWN => '?',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DirectionEnum {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

impl DirectionEnum {
    pub const ALL: [DirectionEnum; 4] = [
        DirectionEnum::UP,
        DirectionEnum::DOWN,
        DirectionEnum::LEFT,
        DirectionEnum::RIGHT,
    ];
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn step(self, direction: DirectionEnum) -> Position {
        match direction {
            DirectionEnum::UP => Position::new(self.x, self.y - 1),
            DirectionEnum::LEFT => Position::new(self.x - 1, self.x),
            DirectionEnum::DOWN => Position::new(self.x, self.y + 1),
            DirectionEnum::RIGHT => Position::new(self.x + 1, self.x),
        }
    }

    pub fn distance_square(self, position: Position) -> i32 {
        (self.x - position.x) * (self.x - position.x)
            + (self.y - position.y) * (self.y - position.y)
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub size: i32,
    tiles: Vec<TileEnum>,
}

impl Grid {
    pub fn new(size: i32) -> Grid {
        Grid {
            size,
            tiles: vec![TileEnum::EMPTY; size as usize * size as usize],
        }
    }

    pub fn in_bounds(&self, position: Position) -> bool {
        position.x >= 0 && position.y >= 0 && position.x < self.size && position.y < self.size
    }

    pub fn get(&self, position: Position) -> TileEnum {
        if self.in_bounds(position) {
            self.tiles[self.index_of(position)]
        } else {
            TileEnum::UNKNOWN
        }
    }

    pub fn set(&mut self, position: Position, tile: TileEnum) {
        if self.in_bounds(position) {
            let index = self.index_of(position);
            self.tiles[index] = tile;
        }
    }

    pub fn walkable(&mut self, position: Position) -> bool {
        self.get(position) == TileEnum::EMPTY
    }

    pub fn print(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                let t = self.tiles[(y * self.size + x) as usize];
                print!("{} ", t.symbol());
            }
            println!();
        }
    }

    pub fn set_random_rocks(&mut self, count: usize, seed: u64) {
        let mut state = seed.max(1); // must not be 0
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };

        // never fill every cell: keep (0,0) free for the player
        let count = count.min((self.size * self.size - 1) as usize);
        let mut placed = 0;
        while placed < count {
            let x = (next() % self.size as u64) as i32;
            let y = (next() % self.size as u64) as i32;
            let pos = Position::new(x, y);

            if (x == 0 && y == 0) || self.get(pos) == TileEnum::ROCK || self.get(pos) == TileEnum::START || self.get(pos) == TileEnum::END {
                continue; // start cell or already a rock: try again
            }
            self.set(pos, TileEnum::ROCK);
            placed += 1;
        }
    }

    fn index_of(&self, position: Position) -> usize {
        (position.x * self.size + position.y) as usize
    }
}
