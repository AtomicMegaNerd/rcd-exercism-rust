// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
use Direction::*;

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        match self.d {
            North => Self { d: East, ..self },
            East => Self { d: South, ..self },
            South => Self { d: West, ..self },
            West => Self { d: North, ..self },
        }
    }

    pub fn turn_left(self) -> Self {
        match self.d {
            North => Self { d: West, ..self },
            West => Self { d: South, ..self },
            South => Self { d: East, ..self },
            East => Self { d: North, ..self },
        }
    }

    pub fn advance(self) -> Self {
        match self.d {
            North => Self {
                y: self.y + 1,
                ..self
            },
            West => Self {
                x: self.x - 1,
                ..self
            },
            East => Self {
                x: self.x + 1,
                ..self
            },
            South => Self {
                y: self.y - 1,
                ..self
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        // Folds are fun as they remind me of Haskell.  Also, this is totally
        // immutable.
        instructions.chars().fold(self, |robot, step| match step {
            'R' => robot.turn_right(),
            'L' => robot.turn_left(),
            'A' => robot.advance(),
            _ => robot,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
