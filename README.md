# Rust CLI To-Do List

A simple command-line to-do list application written in Rust. The application reads tasks from a file (default: `.tasklist`) in CSV-like format, and allows you to manage your tasks via various commands.

---

## Features

- **Add** a new task
- **Update** an existing task’s status or description
- **Show** tasks in a list or a Kanban-like board
- **Delete** a task

---

## Installation

1. **Clone** this repository or copy the source files.
2. **Install** [Rust and Cargo](https://www.rust-lang.org/tools/install) if you haven’t already.
3. **Build** and install the CLI:

```
cargo install --path .
```

This will compile the project and place a `todolist` binary in your Cargo bin directory (e.g., `~/.cargo/bin`).

---

## Usage

```
todolist [COMMAND] [FLAGS]
```

### Commands

1. **Add** a new task

```
todolist add --description "Task description"
```

2. **Update** an existing task’s status or description

```
todolist update --id <TASK_ID> --status <done|in_progress|not_started> [--description "New description"]
```

3. **Show** the task list

```
todolist show
```

- To show a simple Kanban board view:

```
todolist show --kanban
```

4. **Delete** a task

```
todolist delete --id <TASK_ID>
```

### Global Arguments

- `--file <PATH>` (optional): Specify a custom task file.  
  Defaults to:
  - `.tasklist` in current directory if found
  - otherwise `~/.tasklist` in your home directory

---

## Tasklist File Format

The CLI stores tasks in a file called `.tasklist` by default. Each line represents one task with fields separated by commas. For example:

```
1,✅ Done,Write initial docs
2,⏳ In Progress,Implement feature X
3,🚀 Not Started,Add TUI support
```

---

## Example

1. **Add a new task**:

```
todolist add --description "Finish reading Rust book"
```

2. **Check tasks**:

```
todolist show
```

3. **Update the task**:

```
todolist update --id 1 --status in_progress --description "Finish reading Rust book by Sunday"
```

4. **Show tasks in Kanban view**:

```
todolist show --kanban
```

---

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

1. Fork this repository
2. Create a new branch
3. Commit changes
4. Create a pull request

---

## License

This project is licensed under the [MIT License](LICENSE).

---

**Enjoy your Rust CLI To-Do List!**
