# wtx

A CLI tool for managing Git worktrees and VSCode/Kiro workspaces.

[日本語](README.ja.md)

## Features

- Centralized management of multiple repositories via bare clone
- Automatic generation of worktrees and workspace files
- Accelerate parallel development setup

## Installation

```bash
cargo install --path .
```

## Usage

### Register repositories

```bash
wtx register git@github.com:org/frontend.git
wtx register git@github.com:org/backend.git
```

Registered repositories are bare cloned to `~/.wtx/`.

### List registered repositories

```bash
wtx list
```

### Create a workspace

```bash
cd ~/work
wtx new feature-auth
```

Select repositories and branches interactively, then a `feature-auth/` directory will be created containing worktrees and a `.code-workspace` file.

### Unregister a repository

```bash
wtx unregister frontend
```

## Data Location

```
~/.wtx/
├── config.json        # Registered repositories
├── frontend.git/      # Bare repository
└── backend.git/       # Bare repository
```

## Development

```bash
# Build
cargo build

# Test
cargo test

# Release build
cargo build --release
```

## License

MIT License - Copyright (c) mzkmnk <mzk.mnk.dev@gmail.com>
