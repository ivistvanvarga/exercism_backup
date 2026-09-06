#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
}

enum GameState {
    Incomplete,
    Complete,
    Invalid,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { rolls: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if matches!(game_state(&self.rolls), GameState::Complete) {
            return Err(Error::GameComplete);
        }

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let mut candidate = self.rolls.clone();
        candidate.push(pins);
        if matches!(game_state(&candidate), GameState::Invalid) {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.rolls = candidate;
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !matches!(game_state(&self.rolls), GameState::Complete) {
            return None;
        }

        let mut score = 0;
        let mut index = 0;
        for _ in 0..10 {
            if self.rolls[index] == 10 {
                score += 10 + self.rolls[index + 1] + self.rolls[index + 2];
                index += 1;
            } else if self.rolls[index] + self.rolls[index + 1] == 10 {
                score += 10 + self.rolls[index + 2];
                index += 2;
            } else {
                score += self.rolls[index] + self.rolls[index + 1];
                index += 2;
            }
        }
        Some(score)
    }
}

fn game_state(rolls: &[u16]) -> GameState {
    let mut index = 0;

    for _ in 0..9 {
        if index >= rolls.len() {
            return GameState::Incomplete;
        }
        if rolls[index] == 10 {
            index += 1;
        } else {
            if index + 1 >= rolls.len() {
                return GameState::Incomplete;
            }
            if rolls[index] + rolls[index + 1] > 10 {
                return GameState::Invalid;
            }
            index += 2;
        }
    }

    if index >= rolls.len() {
        return GameState::Incomplete;
    }

    let first = rolls[index];
    if first == 10 {
        if index + 1 >= rolls.len() {
            return GameState::Incomplete;
        }
        let second = rolls[index + 1];
        if second < 10 && index + 2 < rolls.len() && second + rolls[index + 2] > 10 {
            return GameState::Invalid;
        }
        if index + 2 >= rolls.len() {
            return GameState::Incomplete;
        }
        return GameState::Complete;
    }

    if index + 1 >= rolls.len() {
        return GameState::Incomplete;
    }
    let second = rolls[index + 1];
    if first + second > 10 {
        return GameState::Invalid;
    }
    if first + second == 10
        && index + 2 >= rolls.len() {
            return GameState::Incomplete;
        }
    GameState::Complete
}
