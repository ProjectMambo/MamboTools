# MamboTools

[![CI](https://github.com/ProjectMambo/MamboTools/actions/workflows/ci.yml/badge.svg)](https://github.com/ProjectMambo/MamboTools/actions/workflows/ci.yml)
[![Version 0.1.0](https://img.shields.io/badge/version-0.1.0-7a5fff.svg)](https://github.com/ProjectMambo/MamboTools)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)
[![Ratatui 0.29](https://img.shields.io/badge/ratatui-0.29-blue.svg)](https://ratatui.rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

MamboTools is the terminal home for Project Mambo repositories. It shows the official catalog, tells users what is installed, and performs clone, fast-forward update, and confirmed removal workflows without requiring repository URLs or Git command knowledge.

Version `0.1.0` is a focused local repository manager. Its catalog ships with the binary; there is no account, background service, or remote package registry.

## Motivation

Project Mambo users should not need to memorize repository URLs or repeat Git maintenance commands just to discover and manage official projects. MamboTools provides one clear, guarded interface for the common local repository lifecycle.

## Status

MamboTools `0.1.0` supports local installation, fast-forward updates, and confirmed removal for the built-in public catalog. It is intentionally local-first: accounts, background updates, arbitrary third-party catalogs, and remote repository deletion are out of scope.

## User stories

- As a user, I can discover and install an official Mambo project without finding its clone URL.
- As a user, I can see which projects are installed and update them without creating an accidental merge.
- As a cautious user, I receive a clear confirmation and identity check before a local clone is removed.
- As a maintainer, I can add an official repository through one reviewed catalog entry.

## Getting started

### Requirements

- A terminal.
- [Git](https://git-scm.com/) on `PATH`.
- A current stable Rust toolchain when installing from source.

### Install

Install the current Git release:

```bash
cargo install --git https://github.com/ProjectMambo/MamboTools.git --tag v0.1.0
```

Then run:

```bash
mambo-tools
```

## Features

- Browse the complete Project Mambo repository catalog and local install state.
- Clone public repositories over HTTPS.
- Update with `git pull --ff-only`, never an automatic merge commit.
- Delete only the selected managed Git root after explicit confirmation and origin verification.
- Follow XDG storage conventions, with a direct environment-variable override.
- Render through the shared MamboUI shell, theme, panels, and status components.

## Usage

### Keys

| Key | Action |
|---|---|
| `↑`/`k` | Select the previous repository |
| `↓`/`j` | Select the next repository |
| `i` or `Enter` | Install the selected repository |
| `u` | Fast-forward update the selected repository |
| `d` | Request deletion |
| `y` | Confirm a requested deletion |
| `n` or `Esc` | Cancel a requested deletion |
| `r` | Refresh local state |
| `q` or `Ctrl+C` | Quit |

Operations report Git and filesystem failures in the status line. Git prompts are disabled inside the full-screen UI; configure credentials before managing a private repository.

### Repository location

MamboTools selects the first available location:

1. `$MAMBO_REPOS_DIR`;
2. `$XDG_DATA_HOME/mambo/repos`;
3. `$HOME/.local/share/mambo/repos`.

Each catalog entry is installed as one direct child of that root. For example, `MamboUI` is stored at `$XDG_DATA_HOME/mambo/repos/MamboUI` when `XDG_DATA_HOME` is set.

### Safety model

MamboTools never overwrites a pre-existing path. An update uses `git pull --ff-only`; divergent history fails and remains untouched. Before update or deletion, the selected repository's `origin` must identify the matching `ProjectMambo/<Repository>` GitHub repository.

Deletion requires a second keypress, rejects symbolic links and non-Git paths, and asks Git to verify that the selected directory is the repository root before removal. It removes the complete local clone, including untracked and uncommitted files, but never deletes a GitHub repository.

## Documentation

| Goal | Guide |
|---|---|
| Install and operate MamboTools | [User guide](docs/User%20Guide.md) |
| Understand safety and implementation boundaries | [Architecture](docs/Architecture.md) |
| Browse the public Wiki page | [projectmambo.org/mambotools/](https://projectmambo.org/mambotools/) |
| Inspect shared terminal components | [MamboUI](https://github.com/ProjectMambo/MamboUI) |

## Project structure

```text
src/catalog.rs       official repository catalog
src/paths.rs         XDG root and safe managed paths
src/repo.rs          clone, update, identity, and delete boundary
src/main.rs          Ratatui event loop and presentation
docs/                synchronized user and developer guides
.github/workflows/   formatting, test, and Clippy checks
```

## Validation

Run the complete local gate:

```bash
cargo fmt --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --locked
```

## Development

The project intentionally invokes the installed Git executable instead of embedding another Git implementation. Author documentation in `notes/Docs/Projects/MamboTools/`, then synchronize it with `notes/Scripts/sync_docs.js`.

## License

MamboTools is available under the [MIT License](LICENSE).
