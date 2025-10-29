# STree - Smart Tree 🌴
_by Benjamin Grolleau & Angelo Tunney_

![Rust](https://img.shields.io/badge/language-Rust-orange?logo=rust)
![Status](https://img.shields.io/badge/status-in%20development-yellow)

STree is a modern and smart reimplementation of the traditional tree command. Its goal is to provide developers with a more intuitive and visually clear way to explore project structures. Unlike the classic version, STree introduces several improvements such as configurable depth levels for better readability, colorized output to distinguish files and directories at a glance, and automatic filtering of files or folders ignored by Git (based on the .gitignore file). It can also display the Git status of files, showing whether they are untracked, modified, staged for commit, or stashed. STree is written in Rust.

## 🚀 Run in development
You can run *Stree* directly from source with Cargo:
```bash
cargo run -- [Options]
```

Example:
```bash
cargo run -- --depth 2 --color always --icons --git
```

This will display the directory tree up to depth 2, with colors, icons, and Git status indicators.

## 🧪 Run tests
All integration tests are located in `./tests/` directory while unit tests are side by side to the file.

To run them:
```bash
cargo test
```

## 🧰 Build the binary
To build the project in release mode:
```bash
cargo build --release
```

The compiled binary will be located in:
```
./target/release/stree
```

You can then run it directly:
```bash
./target/release/stree [Options]
```

## 🦀 Cli Usages
You can display all available arguments using:
```bash
cargo run -- --help
```

## 🧹 Code Quality & Pre-commit Hooks
STree uses pre-commit to automatically enforce code quality and formatting rules before each commit. It ensures that every commit respects Rust’s formatting and linting standards.

### 🧩 Prerequisites
Install pre-commit using one of the following methods:

```shell
# macOS
brew install pre-commit

# Linux
sudo apt install pre-commit

# With a local Python environment
uv add --dev pre-commit
# or
poetry add --dev pre-commit
```

### ⚙️ Install
Once installed, enable the hooks:
```shell
pre-commit install
```

### 🧪 Run manually
You can manually run all hooks on the repository:
```
pre-commit run --all-files
```

### 🧭 Current state

STree is currently in active development.
All CLI options are **planned**, but some are not yet implemented.

| Option | Status | Description |
|--------|---------|-------------|
| `--gitignore` | ✅ | Show files listed in `.gitignore` (ignored by default) |
| `--hidden-files` | ✅ | Include hidden files and directories (starting with `.`) |
| `--color <Auto\|Always\|Never>` | 🕓 | Colorize the output (directories, files, others) |
| `--icons` | 🕓 | Add icons for known file types and directories |
| `--depth <N>` | ✅ | Limit the displayed depth of the tree |
| `--dirs-only` | 🕓 | Display only directories |
| `--files-only` | 🕓 | Display only files |
| `--prune-empty` | 🕓 | Hide empty directories |
| `--git` | 🕓 | Display Git status indicators (modified, staged, untracked, etc.) |
| `--git-branch` | 🕓 | Show the current Git branch name next to the root |
| `--json` | 🕓 | Output the tree as a JSON structure |
| `--count` | 🕓 | Print only the number of files and directories |
| `--time` | 🕓 | Measure and display execution time |
| `--verbose` | ✅ | Enable detailed logging |

## 👥 Contributors

<table align="center">
  <tr>
    <td align="center">
      <a href="https://github.com/Grolleau-Benjamin">
        <img src="https://images.weserv.nl/?url=avatars.githubusercontent.com/u/127044450?v=4&h=90&w=90&fit=cover&mask=circle" width="90" height="90" alt="Benjamin Grolleau"/><br/>
        <sub><b>Benjamin Grolleau</b></sub>
      </a>
    </td>
    <td align="center">
      <a href="https://github.com/angelo-tny">
        <img src="https://images.weserv.nl/?url=github.com/angelo-tny.png&h=90&w=90&fit=cover&mask=circle" width="90" height="90" alt="Angelo Tunney"/><br/>
        <sub><b>Angelo Tunney</b></sub>
      </a>
    </td>
  </tr>
</table>
