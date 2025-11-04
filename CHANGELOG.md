# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0-alpha] — 2025-11-04
### Added
- `--time` flag to measure and display total execution duration (printed to stderr).
- `--count` flag to print directory and file count.
- Helper function `format_duration()` (src/helpers.rs) to render durations in ns/µs/ms/s.

### Internal
- Unit tests for `format_duration()` and the count renderer.
- Minor refactors in timing and renderer modules for clarity and extensibility.

## [0.4.0-alpha] — 2025-11-03
### Added
- `--color {never,auto,always}` :
  - `never`: no colors
  - `auto`: colorize only Git markers (default)
  - `always`: full colorized output (file type + Git markers)
- Extension-based color table using `phf` (`renderer/colors.rs`).
- Colored Git markers with symbols (~, +, ?, →, ✖).

### Changed
- Reworked stdout renderer with 4 branchless variants:
  - classic
  - colorized
  - icons
  - icons + colorized

### Performance
- Zero allocations in the hot path (direct `Write` calls).
- O(1) lookups for extension colors and Git markers.

### Internal
- New `renderer/colors.rs` module.
- Updated `RenderOptions` and CLI to handle color modes.



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
