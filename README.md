# LifeGame

[![Crates.io](https://img.shields.io/crates/v/lifegame.svg)](https://crates.io/crates/lifegame)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A simple implementation of the classic cellular automaton, Conway's Game of Life.

## Installation

```
cargo install lifegame
```

## Usage

Create a world with 80×100 cells:

```
lifegame --nrows 80 --ncols 100
```

Apply classic *Matrix*-style filter:

```
lifegame --filter bit
```

Emojify the world:

```
lifegame --filter emoji
```

Get help:

```
lifegame --help
```

## Keymap

| Key | Action             |
| --- | ------------------ |
| `j` | Slow down by half  |
| `k` | Speed up by twice  |
| `p` | Toggle pause       |
| `f` | Random flip by 10% |
| `r` | Reset world        |
| `q` | Quit the game      |
