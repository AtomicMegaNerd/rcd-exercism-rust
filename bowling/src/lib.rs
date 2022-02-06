#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const PINS: u16 = 10;

pub struct BowlingGame {
    frames: [Frame; 10],
    rolls: Vec<Roll>,
    curr_frame: usize,
    complete: bool,
}

pub struct Frame {
    is_final: bool,
    bonus_roll: bool,
    curr_roll: usize,
    pins_left: u16,
}

pub struct Roll {
    in_final_frame: bool,
    is_strike: bool,
    is_spare: bool,
    score: u16,
}

impl Frame {
    fn new(is_final: bool) -> Self {
        Frame {
            is_final,
            bonus_roll: false,
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
            rolls: Vec::new(),
            frames: [
                Frame::new(false),
                Frame::new(false),
                Frame::new(false),
                Frame::new(false),
                Frame::new(false),
                Frame::new(false),
                Frame::new(false),
                Frame::new(false),
                Frame::new(false),
                Frame::new(true),
            ],
            curr_frame: 0,
            complete: false,
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
        let is_final_frame = frame.is_final;
        let bonus_roll = frame.bonus_roll;

        // Roll the ball towards the pins!
        frame.curr_roll += 1;

        println!(
            "Frame: {}, Current Roll: {}",
            self.curr_frame, frame.curr_roll,
        );

        // Strike
        if pins == 10 && frame.curr_roll == 1 {
            self.rolls.push(Roll {
                in_final_frame: is_final_frame,
                is_strike: true,
                is_spare: false,
                score: 10,
            });

            if is_final_frame {
                frame.bonus_roll = true;
                frame.pins_left = 10;
            } else {
                frame.pins_left = 0;
            }

            // With a strike advance to the next frame right away
            self.next_frame();
        } else {
            // We cannot knock down more than 10 pins
            if pins > frame.pins_left {
                println!("Not enough pins left...");
                return Err(Error::NotEnoughPinsLeft);
            }

            // if there are no pins left this has to be a spare
            frame.pins_left -= pins;
            let is_spare = frame.pins_left == 0;
            if is_spare && is_final_frame {
                frame.bonus_roll = true;
                frame.pins_left = 10;
            }

            self.rolls.push(Roll {
                in_final_frame: is_final_frame,
                is_strike: false,
                is_spare,
                score: pins,
            });
        }

        if is_final_frame && ((curr_roll >= 3 && bonus_roll) || (curr_roll == 2)) {
            self.complete = true;
            Err(Error::GameComplete)
        } else {
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.complete {
            let mut score = 0;

            // First count all of the rolls not in the final frame.
            // These can have bonuses.
            for roll_window in self.rolls.windows(3) {
                let cur = &roll_window[0];
                let next = &roll_window[1];
                let aft = &roll_window[2];

                if !cur.in_final_frame {
                    if cur.is_strike {
                        score += cur.score + next.score + aft.score;
                    } else if cur.is_spare {
                        score += cur.score + next.score
                    } else {
                        score += cur.score;
                    }
                }
            }

            // This will add up the score from the final frame
            score += self
                .rolls
                .iter()
                .filter(|r| r.in_final_frame)
                .fold(0, |acc, r| acc + r.score);

            Some(score)
        } else {
            None
        }
    }
}
