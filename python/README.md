# LifeGame-TUI

A simple Python implementation of the classic cellular automaton, Conway's Game of Life.

[![PyPI - Version](https://img.shields.io/pypi/v/lifegame-tui.svg)](https://pypi.org/project/lifegame-tui)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/lifegame-tui.svg)](https://pypi.org/project/lifegame-tui)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

-----

**Table of Contents**

- [LifeGame-TUI](#lifegame-tui)
  - [Installation](#installation)
  - [Usage](#usage)
  - [License](#license)

## Installation

```console
pip install lifegame-tui
```

## Usage

Create a lifegame with $80 \times 100$ cells (resize your terminal window to prevent display glitches):

```console
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

`LifeGame-TUI` is distributed under the terms of the [MIT](https://spdx.org/licenses/MIT.html) license.
