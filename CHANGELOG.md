# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0-alpha] —
### Added
- Helper function `format_duration()` (in `src/helpers.rs`) to render durations in a human-readable format (ns, µs, ms, s).

### Changed
- Refined all file type colors to use a **pastel-themed palette** for a softer, more cohesive visual style.

### Consistency
- Color scheme aligned with modern terminal themes (Catppuccin / Tokyo Night Light aesthetic).

### Internal
- Helper function `format_duration()` (in `src/helpers.rs`) to render durations in a human-readable format (ns, µs, ms, s).



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
