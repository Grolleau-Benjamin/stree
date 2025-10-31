# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0-alpha] — 2025-10-31
### Added
- Icons rendering (dir/file/extension) via PHF maps.
- High-performance stdout renderer with single-dispatch closure (no per-node branch).

### Performance
- Reduced allocations in stdout renderer (no format! in hot path, BufWriter-friendly).

### Internal
- Refactor: `render_to` now takes a name builder `Fn(&Node) -> String`.

## [0.1.0-alpha] — 2025-10-30
### Added
- Initial alpha release
