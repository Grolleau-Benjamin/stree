# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0-alpha] — 2025-11-03
### Added
- JSON renderer for structured output (`--json`).

### Changed
- Unified rendering backend (`stdout` / `json`) selected dynamically at runtime.
- `stdout::render` now returns `io::Result<()>` for consistency with other renderers.

### Performance
- Optimized `stdout` renderer to minimize string allocations and prefix construction.
- Maintained BufWriter compatibility for faster terminal output.

### Internal
- New `renderer/json.rs` module with unit tests for structure and Git state serialization.
- Added comprehensive tests for JSON rendering (single node, nested, git states, omitted fields).

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
