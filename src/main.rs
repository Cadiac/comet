use clap::Parser;
use env_logger::Env;
use std::error::Error;

use rayon::prelude::*;

use rand::Rng;

#[macro_use]
extern crate log;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of games to simulate
    #[clap(short, long, value_parser, default_value_t = 100)]
    games: usize,

    /// Print game actions debug output (slow)
    #[clap(short, long, action)]
    verbose: bool,

    /// Should combat damage using squirrels be included
    #[clap(short, long, action)]
    squirrels: bool,

    /// Extra roll advantage effects
    #[clap(short, long, value_parser, default_value_t = 0)]
    advantage: u32,
}

enum Outcome {
    Win,
    Lose,
}

struct GameResult {
    outcome: Outcome,
    damage: u32,
    squirrels: u32,
    rolls: u32,
}

struct Game {
    roll_advantage: u32,
    damage: u32,
    squirrels: u32,
    include_squirrels: bool,
    rolls: u32,
    activations_left: u32,
    loyalty: i32,
}

impl Game {
    fn new(include_squirrels: bool, roll_advantage: u32) -> Self {
        Self {
            roll_advantage,
            activations_left: 1,
            squirrels: 0,
            damage: 0,
            rolls: 0,
            loyalty: 5,
            include_squirrels,
        }
    }

    fn run(&mut self) -> GameResult {
        while self.activations_left > 0 && self.loyalty > 0 && self.damage < 10000 {
            self.activate()
        }

        let outcome = if self.damage >= 20
            || self.include_squirrels && (self.damage + self.squirrels) >= 20
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
        }
    }

    fn activate(&mut self) {
        self.activations_left -= 1;

        let mut rng = rand::thread_rng();

        let rolls_to_take = 1 + self.roll_advantage;
        self.rolls += rolls_to_take;

        log::debug!(
            "[Loyalty: {}][Activations: {}] 0: Roll a six-sided die.",
            self.loyalty,
            self.activations_left
        );

        let mut max_roll = 0;
        for _ in 0..rolls_to_take {
            let roll: u32 = rng.gen_range(1..=6);
            log::debug!("Rolled a {roll}.");

            if roll > max_roll {
                max_roll = roll;
            }
        }

        log::debug!("Kept the max roll {max_roll}.");

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
            log::debug!(
                "[Loyalty: {}][Activations: {}] -1: No action.",
                self.loyalty,
                self.activations_left
            );

        //4 or 5 — Comet, Stellar Pup deals damage equal to the number of loyalty counters on him to a creature or player, then [-2].
        } else if max_roll == 4 || max_roll == 5 {
            self.damage += i32::max(self.loyalty, 0) as u32;
            self.loyalty -= 2;

            log::debug!(
                "[Loyalty: {}][Activations: {}] -2: Comet, Stellar Pup dealt {} damage.",
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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();
    init_logger(cli.verbose);

    let simulated_games = cli.games;

    let results: Vec<_> = (0..simulated_games)
        .into_par_iter()
        .map(|_| {
            let mut game = Game::new(cli.squirrels, cli.advantage);
            game.run()
        })
        .collect();

    let total_wins: usize = results
        .iter()
        .filter(|result| match result.outcome {
            Outcome::Win => true,
            Outcome::Lose => false,
        })
        .count();

    let win_percentage = 100.0 * total_wins as f32 / simulated_games as f32;

    let mut total_damage = 0;
    let mut total_rolls = 0;
    let mut total_squirrels = 0;

    for game in results {
        total_damage += game.damage;
        total_rolls += game.rolls;
        total_squirrels += game.squirrels;
    }

    let average_damage = total_damage as f32 / simulated_games as f32;
    let average_rolls = total_rolls as f32 / simulated_games as f32;
    let average_squirrels = total_squirrels as f32 / simulated_games as f32;

    info!("=======================[ RESULTS ]==========================");
    info!("                 Win percentage: {win_percentage:.2}%");
    info!("                 Average damage: {average_damage:.2}");
    info!("              Average squirrels: {average_squirrels:.2}");
    info!("                  Average rolls: {average_rolls:.2}");
    info!("============================================================");

    Ok(())
}

fn init_logger(verbose: bool) {
    let default_level = if verbose { "debug" } else { "info" };

    env_logger::Builder::from_env(
        Env::default()
            .filter_or("LOG_LEVEL", default_level)
            .write_style_or("LOG_STYLE", "always"),
    )
    .format_timestamp(None)
    .format_module_path(false)
    .init();
}
