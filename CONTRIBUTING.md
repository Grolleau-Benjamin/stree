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

## License
By contributing, you agree that your contributions will be licensed under the **Apache License 2.0**.
