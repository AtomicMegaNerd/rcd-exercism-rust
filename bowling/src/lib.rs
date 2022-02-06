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
            pins_left: PINS,
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

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // if the game is over do not accept new rolls
        if self.complete {
            return Err(Error::GameComplete);
        }

        // You cannot knock down more pins than are setup
        if pins > PINS {
            return Err(Error::NotEnoughPinsLeft);
        }

        // Our frames array is 0-indexed
        let frame = &mut self.frames[self.curr_frame];

        // Roll the ball towards the pins!
        frame.curr_roll += 1;

        // We cannot knock down more than 10 pins
        if pins > frame.pins_left {
            return Err(Error::NotEnoughPinsLeft);
        }

        // Strikes occur on the first roll of any frame or any roll
        // in the final frame.
        if pins == PINS && (frame.curr_roll == 1 || frame.is_final) {
            self.rolls.push(Roll {
                in_final_frame: frame.is_final,
                is_strike: true,
                is_spare: false,
                score: PINS,
            });

            if frame.is_final {
                frame.bonus_roll = true;
                frame.pins_left = PINS;
            } else {
                frame.pins_left = 0;
                self.curr_frame += 1;
            }
        } else {
            // if there are no pins left this has to be a spare
            frame.pins_left -= pins;
            let is_spare = frame.pins_left == 0;

            if is_spare && frame.is_final {
                frame.bonus_roll = true;
                frame.pins_left = PINS;
            }

            self.rolls.push(Roll {
                in_final_frame: frame.is_final,
                is_strike: false,
                is_spare,
                score: pins,
            });

            if !frame.is_final && frame.curr_roll == 2 {
                self.curr_frame += 1;
            }
        }

        if frame.is_final
            && ((frame.curr_roll == 3 && frame.bonus_roll)
                || (frame.curr_roll == 2 && !frame.bonus_roll))
        {
            self.complete = true;
        }
        Ok(())
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
