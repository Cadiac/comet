# Comet, Stellar Pup simulator

How likely does [Comet, Stellar Pup](https://scryfall.com/card/unf/166/comet-stellar-pup) activation win in a game of Magic: The Gathering with [Pixie Guide](https://scryfall.com/card/afr/66/pixie-guide) like effects on the battlefield?

Comet, Stellar Pup             |  Pixie Guide
:-------------------------:|:-------------------------:
![](https://c1.scryfall.com/file/scryfall-cards/normal/front/a/7/a76fa8d4-923d-4afc-ba47-ba10fc0fe46e.jpg?1663720554)  |  ![](https://c1.scryfall.com/file/scryfall-cards/normal/front/c/6/c65631b9-ca62-4851-9eca-9c760fb1a177.jpg?1627756212)

This tool can also be used to calculate probability of winning from any starting game state, like to find the probability of dealing at least 11 damage with Comet already at 9 loyalty (7.39%).

## Results

| Additional roll effects | Win %  | Squirrels | Damage  | Returns | Total rolls |
|-------------------------|--------|-----------|---------|---------|-------------|
| 0                       | 0.75 % | 1.00      | 2.75    | 0.25    | 1.50        |
| 1                       | 5.87 % | 0.56      | 6.37    | 0.35    | 5.07        | 
| 2                       | 16.3 % | 0.35      | 12.06   | 0.41    | 14.09       |
| 3                       | 29.6 % | 0.22      | 23.55   | 0.45    | 35.74       |
| 4                       | 42.8 % | 0.18      | 76.87   | 0.60    | 111.09      |
| 5                       | 54.1 % | 3.09      | 1971.14 | 16.14   | 6790.02     |
| 6                       | 63.2 % |           |         |         |             |
| 7                       | 70.4 % |           |         |         |             |
| 8                       | 76.3 % |           |         |         |             |
| 9                       | 81.0 % |           |         |         |             |
| 10                      | 84.5 % |           |         |         |             |
| 15                      | 94.3 % |           |         |         |             |
| 20                      | 97.8 % |           |         |         |             |

On higher roll advantage (>5) most of the games where initial rolls succeed go "infinite", so the simulation cuts off when 10000 damage has been dealt and the total stats aren't accurate.

## Installation

Follow [Rust](https://www.rust-lang.org/en-US/install.html) installation instructions.

## Usage

```console
USAGE:
    comet [OPTIONS]

OPTIONS:
    -a, --advantage <ADVANTAGE>    Extra roll advantage effects [default: 0]
    -d, --damage <DAMAGE>          Target damage to deal [default: 20]
    -g, --games <GAMES>            Number of games to simulate [default: 100]
    -l, --loyalty <LOYALTY>        Starting loyalty of the planeswalker [default: 5]
    -s, --squirrels                Should combat damage using squirrels be included
    -h, --help                     Print help information
    -v, --verbose                  Print game actions debug output (slow)
    -V, --version                  Print version information

```

### Examples

```
➜ cargo run --bin comet -- --games 100000 --squirrels --advantage 0
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 0.75%
[INFO ]                  Average damage: 2.75
[INFO ]               Average squirrels: 1.00
[INFO ]                   Average rolls: 1.50
[INFO ]                 Average returns: 0.24
[INFO ] ============================================================

➜ cargo run --bin comet -- --games 100000 --squirrels --advantage 1
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 5.85%
[INFO ]                  Average damage: 6.33
[INFO ]               Average squirrels: 0.57
[INFO ]                   Average rolls: 5.05
[INFO ]                 Average returns: 0.35
[INFO ] ============================================================

➜ cargo run --bin comet -- --games 100000 --advantage 1 --verbose
[DEBUG] [Loyalty: 5][Activations: 0] 0: Roll a six-sided die.
[DEBUG] Rolled a 6.
[DEBUG] Rolled a 1.
[DEBUG] Kept the max roll of 6.
[DEBUG] [Loyalty: 6][Activations: 2] +1: Two extra activations.
[DEBUG] [Loyalty: 6][Activations: 1] 0: Roll a six-sided die.
[DEBUG] Rolled a 6.
[DEBUG] Rolled a 4.
[DEBUG] Kept the max roll of 6.
[DEBUG] [Loyalty: 7][Activations: 3] +1: Two extra activations.
[DEBUG] [Loyalty: 7][Activations: 2] 0: Roll a six-sided die.
[DEBUG] Rolled a 2.
[DEBUG] Rolled a 5.
[DEBUG] Kept the max roll of 5.
[DEBUG] [Loyalty: 5][Activations: 2] -2: Comet, Stellar Pup deals 7 damage.
[DEBUG] [Loyalty: 5][Activations: 1] 0: Roll a six-sided die.
[DEBUG] Rolled a 6.
[DEBUG] Rolled a 2.
[DEBUG] Kept the max roll of 6.
[DEBUG] [Loyalty: 6][Activations: 3] +1: Two extra activations.
[DEBUG] [Loyalty: 6][Activations: 2] 0: Roll a six-sided die.
[DEBUG] Rolled a 4.
[DEBUG] Rolled a 6.
[DEBUG] Kept the max roll of 6.
[DEBUG] [Loyalty: 7][Activations: 4] +1: Two extra activations.
[DEBUG] [Loyalty: 7][Activations: 3] 0: Roll a six-sided die.
[DEBUG] Rolled a 4.
[DEBUG] Rolled a 2.
[DEBUG] Kept the max roll of 4.
[DEBUG] [Loyalty: 5][Activations: 3] -2: Comet, Stellar Pup deals 7 damage.
[DEBUG] [Loyalty: 5][Activations: 2] 0: Roll a six-sided die.
[DEBUG] Rolled a 5.
[DEBUG] Rolled a 5.
[DEBUG] Kept the max roll of 5.
[DEBUG] [Loyalty: 3][Activations: 2] -2: Comet, Stellar Pup deals 5 damage.
[DEBUG] [Loyalty: 3][Activations: 1] 0: Roll a six-sided die.
[DEBUG] Rolled a 4.
[DEBUG] Rolled a 6.
[DEBUG] Kept the max roll of 6.
[DEBUG] [Loyalty: 4][Activations: 3] +1: Two extra activations.
[DEBUG] [Loyalty: 4][Activations: 2] 0: Roll a six-sided die.
[DEBUG] Rolled a 5.
[DEBUG] Rolled a 2.
[DEBUG] Kept the max roll of 5.
[DEBUG] [Loyalty: 2][Activations: 2] -2: Comet, Stellar Pup deals 4 damage.
[DEBUG] [Loyalty: 2][Activations: 1] 0: Roll a six-sided die.
[DEBUG] Rolled a 1.
[DEBUG] Rolled a 6.
[DEBUG] Kept the max roll of 6.
[DEBUG] [Loyalty: 3][Activations: 3] +1: Two extra activations.
[DEBUG] [Loyalty: 3][Activations: 2] 0: Roll a six-sided die.
[DEBUG] Rolled a 5.
[DEBUG] Rolled a 6.
[DEBUG] Kept the max roll of 6.
[DEBUG] [Loyalty: 4][Activations: 4] +1: Two extra activations.
[DEBUG] [Loyalty: 4][Activations: 3] 0: Roll a six-sided die.
[DEBUG] Rolled a 1.
[DEBUG] Rolled a 5.
[DEBUG] Kept the max roll of 5.
[DEBUG] [Loyalty: 2][Activations: 3] -2: Comet, Stellar Pup deals 4 damage.
[DEBUG] [Loyalty: 2][Activations: 2] 0: Roll a six-sided die.
[DEBUG] Rolled a 3.
[DEBUG] Rolled a 3.
[DEBUG] Kept the max roll of 3.
[DEBUG] [Loyalty: 1][Activations: 2] -1: No action. ("Return a card with mana value 2 or less from your graveyard to your hand.")
[DEBUG] [Loyalty: 1][Activations: 1] 0: Roll a six-sided die.
[DEBUG] Rolled a 2.
[DEBUG] Rolled a 3.
[DEBUG] Kept the max roll of 3.
[DEBUG] [Loyalty: 0][Activations: 1] -1: No action. ("Return a card with mana value 2 or less from your graveyard to your hand.")
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 100.00%
[INFO ]                  Average damage: 27.00
[INFO ]               Average squirrels: 0.00
[INFO ]                   Average rolls: 28.00
[INFO ]                 Average returns: 2.00
[INFO ] ============================================================
```

## License

This project is released under [MIT](https://github.com/Cadiac/comet/blob/master/LICENSE) license.

[PicoCSS](https://picocss.com/), a minimal CSS Framework for semantic HTML is used for styles. A copy of the minimized css provided by the framework is included inside `vendor/pico` directory.

The literal and graphical information presented on this site about Magic: The Gathering, including the card image `static/comet.jpeg`, the mana symbols, and Oracle text, is copyright Wizards of the Coast, LLC, a subsidiary of Hasbro, Inc. This service is not affiliated with Wizards of the Coast.
