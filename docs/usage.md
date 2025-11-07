# 🦀 CLI Usage & Options

Arbor is a modern reimplementation of the `tree` command written in Rust.
It supports `.gitignore`, colorized output, Git integration, icons, JSON export, and more.

## 🚀 Run from source

```bash
cargo run -- [options]
```

Example:

```bash
cargo run -- --depth 2 --color always --icons --git
```

## 🔧 Available options

| Option | Short | Status | Description |
|--------|--------|---------|-------------|
| `--show-gitignored` | `-G` | ✅ | Show files listed in `.gitignore` (ignored by default) |
| `--show-hiddens` | `-H` | ✅ | Include hidden files and directories (starting with `.`) |
| `--color <Auto\|Always\|Never>` | `-c <>` | ✅ | Colorize the output (directories, files, others) |
| `--icons` | `-i` | ✅ | Add icons for known file types and directories |
| `--depth <N>` | `-d <N>` | ✅ | Limit the displayed depth of the tree |
| `--git` | `-g` | 🕓 | Display Git status indicators (modified, staged, untracked, etc.) |
| `--git-branch` | `-b` | 🕓 | Show the current Git branch name next to the root |
| `--json` | `-j` | ✅ | Output the tree as a JSON structure |
| `--count` | `-n` | ✅ | Print only the number of files and directories |
| `--time` | `-t` | ✅ | Measure and display execution time |
| `--verbose` | `-v` | ✅ | Enable detailed logging |

## 🧪 Testing

```bash
cargo test
```

Unit tests are inline while integration tests live in ./tests/.
