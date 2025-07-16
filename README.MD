```markdown
# CLI Todo Application

A simple command-line todo application written in Rust, featuring task management with deadlines, CSV persistence, and a clean tabular display.

## Features

- **Add tasks** with optional deadlines
- **List tasks** in a formatted table with status indicators
- **Update tasks** (mark as done)
- **Remove tasks** by ID
- **Save/Load tasks** to/from CSV files
- **Cross-platform** terminal clearing
- **Interactive prompt** with command suggestions

## Installation

1. Ensure you have Rust installed (install via [rustup](https://rustup.rs/))
2. Clone this repository
3. Build the application:
   ```bash
   cargo build --release
   ```
4. Run the executable:
   ```bash
   ./target/release/cli_todo
   ```

## Usage

### Commands

| Command      | Description                          |
|--------------|--------------------------------------|
| `add`        | Add a new task                       |
| `list task`  | Show all tasks in a table            |
| `update`     | Toggle task completion status        |
| `remove`     | Delete a task by ID                  |
| `save`       | Save tasks to tasks.csv              |
| `load tasks` | Load tasks from a CSV file           |
| `clear`/`cls`| Clear the terminal screen            |
| `help`/`?`   | Show available commands              |
| `exit`       | Quit the application                 |

### Task Structure

Each task contains:
- Title (required)
- Creation timestamp (auto-generated)
- Deadline (optional)
- Completion status (default: false)

### CSV Format

Tasks are saved in CSV format with these columns:
1. Task title
2. Creation timestamp
3. Deadline (empty if none)
4. Completion status (true/false)

## Dependencies

- `chrono` - For timestamp handling
- `csv` - For file persistence
- `prettytable` - For formatted task display

## Examples

```bash
> add
Enter your task:
Buy groceries
Enter deadline (or press enter to skip):
2023-12-31
Task added ✅

> list task
+---+-------------+----------------------------+------------+--------+
| # |    Title    |         Created At         |  Deadline  | Status |
+---+-------------+----------------------------+------------+--------+
| 1 | Buy groceries | 2023-11-15 14:30:45.123456 | 2023-12-31 |   ❌   |
+---+-------------+----------------------------+------------+--------+

> update
[shows task list]
Enter the task number you want to update:
1
✅ Task marked as done!
```

## License

MIT License - Free to use and modify
```

This README includes:
1. Clear project description
2. Installation instructions
3. Comprehensive command documentation
4. File format specification
5. Dependency information
6. Usage examples
7. License information

You may want to customize the license section if you're using a different license, and add your contact information if you want users to be able to reach you with questions.