# Comet, Stellar Pup simulator

How likely does [Comet, Stellar Pup](https://scryfall.com/card/unf/166/comet-stellar-pup) activation win in a game of Magic: The Gathering with [Pixie Guide](https://scryfall.com/card/afr/66/pixie-guide) like effects on the battlefield?

![Comet, Stellar Pup](https://c1.scryfall.com/file/scryfall-cards/normal/front/a/7/a76fa8d4-923d-4afc-ba47-ba10fc0fe46e.jpg?1663720554)
![Pixie Guide](https://c1.scryfall.com/file/scryfall-cards/normal/front/c/6/c65631b9-ca62-4851-9eca-9c760fb1a177.jpg?1627756212)

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


## Results

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
➜ ./comet --games 100000 --squirrels --advantage 2
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 16.38%
[INFO ]                  Average damage: 12.07
[INFO ]               Average squirrels: 0.35
[INFO ]                   Average rolls: 14.07
[INFO ] ============================================================
➜ ./comet --games 100000 --squirrels --advantage 3
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 29.88%
[INFO ]                  Average damage: 23.74
[INFO ]               Average squirrels: 0.22
[INFO ]                   Average rolls: 36.05
[INFO ] ============================================================
➜ ./comet --games 100000 --squirrels --advantage 4
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 42.75%
[INFO ]                  Average damage: 77.45
[INFO ]               Average squirrels: 0.19
[INFO ]                   Average rolls: 111.40
[INFO ] ============================================================
➜ ./comet --games 100000 --squirrels --advantage 5
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 54.31%
[INFO ]                  Average damage: 1320.09
[INFO ]               Average squirrels: 0.38
[INFO ]                   Average rolls: 842.60
[INFO ] ============================================================
➜ ./comet --games 100000 --squirrels --advantage 6
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 63.25%
[INFO ]                  Average damage: 4710.12
[INFO ]               Average squirrels: 0.27
[INFO ]                   Average rolls: 2031.81
[INFO ] ============================================================
➜ ./comet --games 100000 --squirrels --advantage 7
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 70.72%
[INFO ]                  Average damage: 6528.80
[INFO ]               Average squirrels: 0.10
[INFO ]                   Average rolls: 2682.23
[INFO ] ============================================================
```