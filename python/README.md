# LifeGame-TUI

[![PyPI - Version](https://img.shields.io/pypi/v/lifegame-tui.svg)](https://pypi.org/project/lifegame-tui)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/lifegame-tui.svg)](https://pypi.org/project/lifegame-tui)

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

Create a lifegame with $80 \times 100$ cells (please resize your terminal window in case of display glitches):

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

```console
# save
lifegame --hide-stats --iteration-max 1000 > some_file

# load
cat some_file || type fome_file
```

## License

`LifeGame-TUI` is distributed under the terms of the [MIT](https://spdx.org/licenses/MIT.html) license.
