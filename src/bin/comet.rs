use clap::Parser;
use env_logger::Env;
use std::error::Error;

use rayon::prelude::*;

use comet::game::{Outcome, Game};

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
    advantage: usize,

    /// Starting loyalty of the planeswalker
    #[clap(short, long, value_parser, default_value_t = 5)]
    loyalty: i32,

    /// Target damage to deal
    #[clap(short, long, value_parser, default_value_t = 20)]
    damage: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();
    init_logger(cli.verbose);

    let simulated_games = cli.games;

    let results: Vec<_> = (0..simulated_games)
        .into_par_iter()
        .map(|_| {
            let mut game = Game::new(cli.squirrels, cli.advantage, cli.loyalty, cli.damage);
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
    let mut total_returns = 0;

    for game in results {
        total_damage += game.damage;
        total_rolls += game.rolls;
        total_squirrels += game.squirrels;
        total_returns += game.returns;
    }

    let average_returns = total_returns as f32 / simulated_games as f32;
    let average_damage = total_damage as f32 / simulated_games as f32;
    let average_rolls = total_rolls as f32 / simulated_games as f32;
    let average_squirrels = total_squirrels as f32 / simulated_games as f32;

    info!("=======================[ RESULTS ]==========================");
    info!("                 Win percentage: {win_percentage:.2}%");
    info!("                 Average damage: {average_damage:.2}");
    info!("              Average squirrels: {average_squirrels:.2}");
    info!("                  Average rolls: {average_rolls:.2}");
    info!("                Average returns: {average_returns:.2}");
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
