# rpkg

`rpkg` (R Package) is a lightweight CLI wrapper around `Rscript` written in [Rust](https://www.rust-lang.org/) that simplifies installing R packages through terminal commands.

## Features

- Install single or multiple packages in one command
- Default CRAN mirror set to `https://cloud.r-project.org/`
- Optionally select CRAN mirrors by country with fuzzy matching
- Interactive numbered repository selection when there are multiple country matches
- Non-interactive auto-selection of the best match when requested
- Install from explicit git source flags (Github, Gitlab, Codeberg, Bitbucket)
- Optionally install to a library path with `-l` / `--library`

## Prerequisites

- Rust toolchain (e.g., via [rustup](https://rustup.rs/))
- R with `Rscript` available on your `PATH`

## Installation

```bash
cargo install rpkg
```

## Usage

```text
rpkg [OPTIONS] <PACKAGE> [<PACKAGE> ...]
```

Examples:

```bash
# Install a single package from CRAN
rpkg ggplot2

# Install multiple CRAN packages at once
rpkg ggplot2 lubridate mlr3

# Select a CRAN mirror by country (fuzzy matched), then select one between matches
rpkg --country brazil dplyr

# Select by country but auto-pick best match without prompting
rpkg --country brazil --non-interactive dplyr
```

> [!IMPORTANT]
>  **Mirror resolution precedence**
>
> 1. If `--country` is set, `rpkg` retrieves CRAN mirrors through R and asks for selection when needed.
> 2. Else `rpkg` falls back to `https://cloud.r-project.org/`.

```bash
# Install from GitHub source flag
rpkg --github=albersonmiranda/fio

# Mix multiple explicit git sources in one command (no flag means CRAN)
rpkg aCranPack --github=OWNER/REPO --gitlab=OWNER/REPO anotherCranPack -c brazil

# Install into a specific library path
rpkg -l /path/to/R/library ggplot2
```

> [!IMPORTANT]
> **Git source behavior**
> 1. Positional packages are always installed from CRAN.
> 2. Git installs happen only through explicit source flags (`--github`, `--gitlab`, `--bitbucket`, `--codeberg`).
> 3. Source flag values must be in `OWNER/REPO` format.
> 4. Each source flag can be repeated to install multiple repositories.

Options:

```text
-c, --country <COUNTRY>         Country query used for CRAN mirror fuzzy matching
--non-interactive               Auto-select best mirror when multiple matches are found
-l, --library <LIBRARY>         Path to install package library (optional)
--github <OWNER/REPO>           Install from GitHub (repeatable)
--gitlab <OWNER/REPO>           Install from GitLab (repeatable)
--bitbucket <OWNER/REPO>        Install from Bitbucket (repeatable)
--codeberg <OWNER/REPO>         Install from Codeberg (repeatable)
-h, --help                      Print help information
-V, --version                   Print version information
```

## Uninstall
If installed via Cargo:
```bash
cargo uninstall rpkg
```

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to open a PR or issue.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
