#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const PINS: u16 = 10;
const FRAMES: usize = 10;

pub struct BowlingGame {
    frames: [Frame; 10],
    curr_frame: usize,
}

pub struct Frame {
    rolls: Vec<Roll>,
    curr_roll: usize,
    pins_left: u16,
    score: u16,
}

pub struct Roll {
    is_strike: bool,
    is_spare: bool,
}

impl Frame {
    fn new() -> Self {
        Frame {
            rolls: Vec::new(),
            score: 0,
            pins_left: 10,
            curr_roll: 0,
        }
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: [
                Frame::new(),
                Frame::new(),
                Frame::new(),
                Frame::new(),
                Frame::new(),
                Frame::new(),
                Frame::new(),
                Frame::new(),
                Frame::new(),
                Frame::new(),
            ],
            curr_frame: 0,
        }
    }

    pub fn next_frame(&mut self) {
        if self.curr_frame < FRAMES {
            self.curr_frame += 1;
            println!("Setup frame {}", self.curr_frame);
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // You cannot knock down more pins than are setup
        if pins > PINS {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.curr_frame == FRAMES {
            return Err(Error::GameComplete);
        }

        let curr_roll = self.frames[self.curr_frame].curr_roll;
        if curr_roll == 2 {
            self.next_frame();
        }

        // Our frames array is 0-indexed
        let frame = &mut self.frames[self.curr_frame];

        // Roll the ball towards the pins!
        frame.curr_roll += 1;
        println!("Roll #{} in Frame #{}", frame.curr_roll, self.curr_frame);

        // Parse the results
        // Strike
        if pins == 10 {
            frame.rolls.push(Roll {
                is_strike: true,
                is_spare: false,
            });
            frame.score += 10;
            frame.pins_left = 0;
            // With a strike advance to the next frame right away
            self.next_frame();
        } else {
            // We cannot knock down more than 10 pins
            if pins + frame.pins_left > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
            frame.pins_left = PINS - pins;
            frame.rolls.push(Roll {
                is_strike: false,
                is_spare: frame.pins_left == 0,
            });
            frame.score += pins;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.curr_frame < 10 {
            None
        } else {
            Some(self.frames.iter().fold(0, |acc, frame| acc + frame.score))
        }
    }
}
