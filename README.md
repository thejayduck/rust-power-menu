# Rust Rofi Power Menu

## Table of Contents

- [About](#about)
- [Dependencies](#dependencies)
- [Installation](#installation)
- [Usage](#usage)
  - [Commands](#commands)

## About
rust-power-menu is a lightweight Rust script that lets you easily access power options through Rofi.

## Dependencies
|Dependency|Link                                              |
|----------|--------------------------------------------------|
|Rofi      |[Github](https://github.com/davatorium/rofi)      |

## Installation
1\. Clone the repository and cd into it.
```
git clone git@github.com:thejayduck/rust-power-menu.git
cd rust-power-menu
```
2\. Build rust-power-menu
```
cargo build --release
```

## Usage
```
rust-power-menu
```

### Commands
| Command                  | Description                                  |
|--------------------------|----------------------------------------------|
| `--no-icon`, `-n`        | Disable icons.                               |
| `--dry-run`, `-d`        | Preview power menu actions without execution.|
| `--help`, `-h`           | Display help information.                    |
| `--version`, `-V`        | Show the version of the script.              |

```
Usage: rust-power-menu [OPTIONS]

Options:
  -n, --no-icon  Disables icons
  -d, --dry-run  Preview power menu actions without execution.
  -h, --help     Print help
  -V, --version  Print version
```