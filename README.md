# Comet, Stellar Pup simulator

How likely does [Comet, Stellar Pup](https://scryfall.com/card/unf/166/comet-stellar-pup) activation win in a game of Magic: The Gathering with [Pixie Guide](https://scryfall.com/card/afr/66/pixie-guide) like effects on the battlefield?

Comet, Stellar Pup             |  Pixie Guide
:-------------------------:|:-------------------------:
![](https://c1.scryfall.com/file/scryfall-cards/normal/front/a/7/a76fa8d4-923d-4afc-ba47-ba10fc0fe46e.jpg?1663720554)  |  ![](https://c1.scryfall.com/file/scryfall-cards/normal/front/c/6/c65631b9-ca62-4851-9eca-9c760fb1a177.jpg?1627756212)

## Results

| Additional roll effects | Win %  | Squirrels | Damage  | Returns | Total rolls |
|-------------------------|--------|-----------|---------|---------|-------------|
| 0                       | 0.75 % | 1.00      | 2.75    | 0.25    | 1.50        |
| 1                       | 5.87 % | 0.56      | 6.37    | 0.35    | 5.07        | 
| 2                       | 16.3 % | 0.35      | 12.06   | 0.41    | 14.09       |
| 3                       | 29.6 % | 0.22      | 23.55   | 0.45    | 35.74       |
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
    -g, --games <GAMES>            Number of games to simulate [default: 100]
    -h, --help                     Print help information
    -s, --squirrels                Should combat damage using squirrels be included
    -v, --verbose                  Print game actions debug output (slow)
    -V, --version                  Print version information

```

### Examples

```
➜ ./comet --games 100000 --squirrels --advantage 0
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 0.75%
[INFO ]                  Average damage: 2.75
[INFO ]               Average squirrels: 1.00
[INFO ]                   Average rolls: 1.50
[INFO ] ============================================================

➜ ./comet --games 100000 --squirrels --advantage 1
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 5.85%
[INFO ]                  Average damage: 6.33
[INFO ]               Average squirrels: 0.57
[INFO ]                   Average rolls: 5.05
[INFO ] ============================================================
```
