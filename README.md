# tasklist

A fast, reliable command-line task manager written in Rust with atomic file operations and file locking.

## Features

- Fast O(1) task addition with metadata caching
- Atomic file operations prevent data corruption
- File locking prevents race conditions
- Kanban board view with terminal width auto-detection
- Interactive TUI mode
- Simple CLI with command aliases

## Installation

### From Source

```bash
cargo install --path .
```

## Usage

### CLI Mode

```bash
# View tasks (default)
tasklist

# Add a task
tasklist add -d "Task description"
tasklist a -d "Task description"  # short alias

# Update task status
tasklist update --id 1 --status in_progress
tasklist u --id 1 --status ip  # with aliases

# Delete task
tasklist delete --id 1
tasklist rm --id 1  # short alias

# Kanban view
tasklist --kanban
tasklist show --kanban
```

### Status Aliases

Use shorter status values:

- `ip` = in_progress
- `d` = done
- `ns` = not_started

### Interactive TUI

Launch the interactive text-based interface:

```bash
tasklist tui
```

**TUI Controls:**

- `↑/k` and `↓/j` - Navigate tasks
- `1/2/3` - Change status (Not Started/In Progress/Done)
- `n` - Add new task
- `d` - Delete task
- `r` - Reload tasks
- `q` or Ctrl+C - Quit

### Global Flags

- `-f, --file <PATH>` - Use custom task file
- `-v, --verbose` - Show verbose output
- `-k, --kanban` - Display Kanban view

## File Format

Tasks are stored in `.tasklist` using tab-separated format:

```
#max_id=3
1 🚀 Not Started Write documentation 2025-12-26 10:00
2 ⏳ In Progress Implement feature 2025-12-26 11:30
3 ✅ Done Fix bug 2025-12-26 09:15
```

## Development

```bash
# Build
cargo build

# Test
cargo test

# Format and lint
cargo fmt
cargo clippy
```

## License

MIT
