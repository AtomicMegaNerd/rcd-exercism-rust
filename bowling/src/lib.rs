#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const PINS: u16 = 10;

pub struct BowlingGame {
    frames: [Frame; 10],
    curr_frame: usize,
}

pub struct Frame {
    rolls: Vec<Roll>,
    curr_roll: usize,
    pins_left: u16,
}

pub struct Roll {
    is_strike: bool,
    is_spare: bool,
    score: u16,
}

impl Frame {
    fn new() -> Self {
        Frame {
            rolls: Vec::new(),
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
        if self.curr_frame < 9 {
            self.curr_frame += 1;
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // You cannot knock down more pins than are setup
        if pins > PINS {
            return Err(Error::NotEnoughPinsLeft);
        }

        let curr_roll = self.frames[self.curr_frame].curr_roll;
        if curr_roll == 2 {
            self.next_frame();
        }

        // Our frames array is 0-indexed
        let frame = &mut self.frames[self.curr_frame];

        // Roll the ball towards the pins!
        frame.curr_roll += 1;

        // Strike
        if pins == 10 && frame.curr_roll == 1 {
            println!("Strike in frame {}", self.curr_frame);
            frame.rolls.push(Roll {
                is_strike: true,
                is_spare: false,
                score: 10,
            });
            frame.pins_left = 0;
            // With a strike advance to the next frame right away
            self.next_frame();
        } else {
            // We cannot knock down more than 10 pins
            if pins > frame.pins_left {
                return Err(Error::NotEnoughPinsLeft);
            }
            // if there are no pins left this has to be a spare
            frame.pins_left -= pins;
            let is_spare = frame.pins_left == 0;

            frame.rolls.push(Roll {
                is_strike: false,
                is_spare,
                score: pins,
            });
        }

        if self.curr_frame == 9 && curr_roll == 2 {
            Err(Error::GameComplete)
        } else {
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.curr_frame >= 9 {
            let mut score = 0;
            let mut strike_bonus = 0;
            let mut spare_bonus = 0;

            for frame in &self.frames {
                for roll in &frame.rolls {
                    println!("Roll score before bonuses {}", roll.score);
                    println!("Strike bonus = {}", strike_bonus);

                    if strike_bonus > 0 {
                        score += roll.score * 2;
                        strike_bonus -= 1;
                    } else if spare_bonus > 0 {
                        score += roll.score * 2;
                        spare_bonus -= 1;
                    } else {
                        score += roll.score;
                    }
                    println!("Score after bonus {}", score);

                    if roll.is_strike {
                        strike_bonus += 2;
                    } else if roll.is_spare {
                        spare_bonus += 1;
                    }
                }
            }
            Some(score)
        } else {
            None
        }
    }
}
