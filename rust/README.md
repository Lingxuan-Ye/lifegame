# LifeGame

A simple Rust implementation of the classic cellular automaton, Conway's Game of Life.


![Rust Version](https://img.shields.io/badge/rust-2021-brightgreen.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

**Table of Contents**

- [LifeGame](#lifegame)
  - [Installation](#installation)
  - [Usage](#usage)
  - [License](#license)

## Installation

Ensure you have Rust and Cargo installed. Then run:

```
cargo install lifegame
```

If you prefer to install from source, run:

```
git clone https://github.com/Lingxuan-Ye/lifegame
cargo install --path ./lifegame/rust/
```

## Usage

Create a LifeGame with $80 \times 100$ cells (resize your terminal window to prevent display glitches):

```
lifegame --nrows 80 --ncols 100
```

Classic *Matrix* style:

```console
lifegame --cell bit
```

Emojify the world:

```console
lifegame --cell emoji
```

Share your game:

```
# save to file
lifegame --hide-stats --iteration-max 1000 > <SOME_FILE>

# load from file
cat <SOME_FILE> || type <SOME_FILE>
```

## License

`LifeGame` is distributed under the terms of the [MIT License](https://spdx.org/licenses/MIT.html).
