# Versioning Policy

STree uses [Semantic Versioning 2.0.0](https://semver.org/).

## Rules

| Change Type | Example | Version Bump | Description |
|--------------|----------|---------------|--------------|
| Breaking change | rename `--depth` to `--max-depth` | **MAJOR** | Not backward compatible |
| New feature | implement behavior for `--gitignore` | **MINOR** | Adds new functionality |
| Bug fix | fix color detection for dark terminals | **PATCH** | Fixes incorrect behavior |

The version is defined in `Cargo.toml` and reflected automatically in the CLI via Clap.

## Release process

To release a new version:
1. Update `Cargo.toml` version.
2. Update `CHANGELOG.md`.
3. Tag the commit with `git tag -s vX.Y.Z -m "Release vX.Y.Z"`.
4. Push tags with `git push --tags`.
