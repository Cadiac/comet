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
[INFO ]                  Win percentage: 0.70%
[INFO ]                  Average damage: 2.72
[INFO ]               Average squirrels: 1.00
[INFO ]                   Average rolls: 1.50
[INFO ] ============================================================
➜ ./comet --games 100000 --squirrels --advantage 1
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 5.19%
[INFO ]                  Average damage: 6.07
[INFO ]               Average squirrels: 0.56
[INFO ]                   Average rolls: 5.00
[INFO ] ============================================================
➜ ./comet --games 100000 --squirrels --advantage 2
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 14.70%
[INFO ]                  Average damage: 10.90
[INFO ]               Average squirrels: 0.32
[INFO ]                   Average rolls: 13.21
[INFO ] ============================================================
➜ ./comet --games 100000 --squirrels --advantage 3
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 28.28%
[INFO ]                  Average damage: 20.70
[INFO ]               Average squirrels: 0.20
[INFO ]                   Average rolls: 32.81
[INFO ] ============================================================
➜ ./comet --games 100000 --squirrels --advantage 4
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 41.74%
[INFO ]                  Average damage: 62.52
[INFO ]               Average squirrels: 0.16
[INFO ]                   Average rolls: 97.38
[INFO ] ============================================================
➜ ./comet --games 100000 --squirrels --advantage 5
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 53.82%
[INFO ]                  Average damage: 1044.19
[INFO ]               Average squirrels: 0.33
[INFO ]                   Average rolls: 729.06
[INFO ] ============================================================
➜ ./comet --games 100000 --squirrels --advantage 6
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 63.28%
[INFO ]                  Average damage: 4577.95
[INFO ]               Average squirrels: 0.26
[INFO ]                   Average rolls: 2014.52
[INFO ] ============================================================
➜ ./comet --games 100000 --squirrels --advantage 7
[INFO ] =======================[ RESULTS ]==========================
[INFO ]                  Win percentage: 70.53%
[INFO ]                  Average damage: 6477.95
[INFO ]               Average squirrels: 0.10
[INFO ]                   Average rolls: 2678.84
[INFO ] ============================================================
```