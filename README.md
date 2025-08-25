A robust and user-friendly command-line interface (CLI) application built with Rust for efficient file and directory management.

**Author:** [Victor Dimah](mailto:imahdavidvictor@gmail.com)

---

## Table of Contents
- [Features](#features)
- [Installation](#installation)
  - [From Source](#from-source)
  - [Using Docker](#using-docker)
- [Usage](#usage)
- [Available Commands](#available-commands)
- [Examples](#examples)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [License](#license)

---

## Features

- **File Operations:** Rename, move, copy, paste, remove, and create files.
- **Directory Operations:** Change, create, and navigate directories.
- **File Search:** Locate files in the current path.
- **File Information:** Display detailed file information.
- **Cross-Platform:** Works on Linux, macOS, and Windows.
- **Docker Support:** Run the CLI in a containerized environment.
- **User-Friendly:** Intuitive commands and helpful error messages.

---

## Installation

### From Source

#### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.60 or higher)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

#### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/victorimah/cli.git
   cd cli
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Install the binary:
   ```bash
   cargo install --path .
   ```
   Or manually copy the binary from `target/release/cli` to a directory in your `PATH`.

### Using Docker

1. Build the Docker image:
   ```bash
   docker build -t victorimah/cli .
   ```
2. Run the CLI in a container:
   ```bash
   docker run -it --rm -v $(pwd):/app victorimah/cli
   ```

---

## Usage

Run the CLI with:
```bash
cli
```
Or with Docker:
```bash
docker run -it --rm -v $(pwd):/app victorimah/cli
```

Use `help` inside the CLI to see all available commands.

---

## Available Commands

| Command                | Description                                      |
|------------------------|--------------------------------------------------|
| `write <old_name> <new_name>` | Rename a file                                    |
| `move <source> <destination>` | Move a file to a new directory                   |
| `locate <file_name>`   | Find a file in the current path                  |
| `cd <destination>`     | Change directory to the specified path           |
| `cd..`                 | Go back to the previous directory                |
| `ima`                  | Update the application (simulated)               |
| `remove <file_name>`   | Remove a specified file                           |
| `info <file_name>`     | Display information about a file                 |
| `copy <file_name>`     | Copy a file (stores in clipboard)                |
| `paste <destination>`  | Paste the copied file to a new location          |
| `new <file_name>`      | Create a new file                                |
| `newd <directory_name>`| Create a new directory                           |
| `ls`                   | List files and directories in the current path   |
| `help`                 | Show this help message                           |
| `exit`                 | Exit the CLI                                     |

---

## Examples

1. Rename a file:
   ```bash
   write old.txt new.txt
   ```
2. Move a file:
   ```bash
   move file.txt /path/to/destination
   ```
3. Locate a file:
   ```bash
   locate example.txt
   ```
4. Create a new directory:
   ```bash
   newd my_folder
   ```
5. Copy and paste a file:
   ```bash
   copy file.txt
   paste /path/to/destination
   ```
6. List files:
   ```bash
   ls
   ```

---

## Configuration

The CLI can be configured using a `.clirc` file in your home directory or project root. Example:

```toml
[default]
verbose = true
```

---

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch:
   ```bash
   git checkout -b feature/your-feature
   ```
3. Commit your changes:
   ```bash
   git commit -m "Add your feature"
   ```
4. Push to the branch:
   ```bash
   git push origin feature/your-feature
   ```
5. Open a pull request.

---