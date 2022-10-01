use rand::{distributions::Uniform, Rng};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Outcome {
    Win,
    Lose,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameResult {
    pub outcome: Outcome,
    pub damage: u32,
    pub squirrels: u32,
    pub rolls: usize,
    pub returns: usize,
}

#[derive(Debug)]
pub struct Game {
    pub rolls: usize,
    pub damage: u32,
    pub squirrels: u32,
    pub loyalty: i32,
    pub activations_left: u32,
    pub returns: usize,
    roll_advantage: usize,
    include_squirrels: bool,
    target_dmg: u32,
}

impl Game {
    pub fn new(include_squirrels: bool, roll_advantage: usize, loyalty: i32, target_dmg: u32) -> Self {
        Self {
            roll_advantage,
            activations_left: 1,
            squirrels: 0,
            damage: 0,
            rolls: 0,
            returns: 0,
            loyalty,
            include_squirrels,
            target_dmg,
        }
    }

    pub fn run(&mut self) -> GameResult {
        while self.activations_left > 0 && self.loyalty > 0 && self.damage < 10000 {
            self.activate()
        }

        let outcome = if self.damage >= self.target_dmg
            || self.include_squirrels && (self.damage + self.squirrels) >= self.target_dmg
        {
            Outcome::Win
        } else {
            Outcome::Lose
        };

        GameResult {
            outcome,
            damage: self.damage,
            squirrels: self.squirrels,
            rolls: self.rolls,
            returns: self.returns,
        }
    }

    pub fn activate(&mut self) {
        if self.loyalty <= 0 || self.activations_left == 0 {
            return
        }

        self.activations_left -= 1;

        let rolls_to_take = 1 + self.roll_advantage;
        self.rolls += rolls_to_take;

        log::debug!(
            "[Loyalty: {}][Activations: {}] 0: Roll a six-sided die.",
            self.loyalty,
            self.activations_left
        );

        let mut rng = rand::thread_rng();
        let die_range = Uniform::new_inclusive::<u32, u32>(1, 6);
        let max_roll = (&mut rng)
            .sample_iter(die_range)
            .take(rolls_to_take)
            .inspect(|roll| log::debug!("Rolled a {roll}."))
            .max()
            .unwrap_or(0);

        log::debug!("Kept the max roll of {max_roll}.");

        // 1 or 2 — [+2], then create two 1/1 green Squirrel creature tokens. They gain haste until end of turn.
        if max_roll == 1 || max_roll == 2 {
            self.loyalty += 2;
            self.squirrels += 2;
            log::debug!(
                "[Loyalty: {}][Activations: {}] +2: Create two 1/1 green Squirrel creature tokens.",
                self.loyalty,
                self.activations_left
            );

        // 3 — [-1], then return a card with mana value 2 or less from your graveyard to your hand.
        } else if max_roll == 3 {
            self.loyalty -= 1;
            self.returns += 1;
            log::debug!(
                "[Loyalty: {}][Activations: {}] -1: No action. (\"Return a card with mana value 2 or less from your graveyard to your hand.\")",
                self.loyalty,
                self.activations_left
            );

        //4 or 5 — Comet, Stellar Pup deals damage equal to the number of loyalty counters on him to a creature or player, then [-2].
        } else if max_roll == 4 || max_roll == 5 {
            self.damage += i32::max(self.loyalty, 0) as u32;
            self.loyalty -= 2;

            log::debug!(
                "[Loyalty: {}][Activations: {}] -2: Comet, Stellar Pup deals {} damage.",
                self.loyalty,
                self.activations_left,
                self.loyalty + 2
            );

        // 6 — [+1], and you may activate Comet, Stellar Pup’s loyalty ability two more times this turn.
        } else if max_roll == 6 {
            self.loyalty += 1;
            self.activations_left += 2;

            log::debug!(
                "[Loyalty: {}][Activations: {}] +1: Two extra activations.",
                self.loyalty,
                self.activations_left
            );
        }
    }
}
