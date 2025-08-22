# rip

`rip` (R Install Package) is a lightweight CLI wrapper around `Rscript` written in [Rust](https://www.rust-lang.org/) that simplifies installing R packages through terminal commands.

## Features

- Install single or multiple packages in one command
- Default CRAN mirror set to `https://cloud.r-project.org/`
- Optionally specify a different repository with `-r` / `--repository`
- Optionally install to a library path with `-l` / `--library`

## Prerequisites

- Rust toolchain (e.g., via [rustup](https://rustup.rs/))
- R with `Rscript` available on your `PATH`

## Installation

Clone or download the repository and then:

```bash
cargo install --path .
```
After installation, the `rip` binary is in your Cargo bin directory (usually `~/.cargo/bin`).

## Usage
```text
rip [OPTIONS] <PACKAGE> [<PACKAGE> ...]
```
Examples:
```bash
# Install a single package
rip ggplot2

# Install multiple packages at once
rip ggplot2 lubridate mlr3

# Use a custom CRAN mirror
rip -r https://cran.rstudio.com/ dplyr

# Install into a specific library path
rip -l /path/to/R/library ggplot2
```

Options:
```text
-r, --repository <REPOSITORY>   CRAN mirror URL [default: https://cloud.r-project.org/]
-l, --library <LIBRARY>         Path to install package library (optional)
-h, --help                      Print help information
-V, --version                   Print version information
```

## Uninstall
If installed via Cargo:
```bash
cargo uninstall rip
```
Or remove the binary manually:
```bash
rm ~/.cargo/bin/rip
```

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to open a PR or issue.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.