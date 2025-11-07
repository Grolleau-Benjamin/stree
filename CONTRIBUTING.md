# Contributing to STree

Thank you for your interest in contributing to STree!

## How to Contribute
1. Fork this repository
2. Create a new branch (`feat/<name>` or `fix/<name>`)
3. Commit your changes following the Conventional Commit style
4. Run formatting and linting checks
   ```bash
   cargo fmt --all -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test
   ```
5. Open a Pull Request with a clear description of your changes

## Code Style
- Use `cargo fmt` for consistent formatting
- Prefer small, focused commits
- Document functions with docstrings when relevant

## 🧹 Code Quality & Pre-commit Hooks
Arbor uses pre-commit to automatically enforce code quality and formatting rules before each commit. It ensures that every commit respects Rust’s formatting and linting standards.

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

## License
By contributing, you agree that your contributions will be licensed under the **Apache License 2.0**.
