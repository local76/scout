# Changelog

## [2026.6.10] - 2026-06-10

### Changed
- **4.2 Path Modernization**: Updated path imports to align with the `library` 4.2 restructured API (using simplified flat namespaces `apps` and `toolkit`).
- **AppData Directory Realignment**: Moved user configuration, database, and log files into a nested %APPDATA%\local76\app\scout structure to organize the ecosystem's configuration space.
- **Repository Rename**: Renamed repository and local directory to app-scout for cleaner ecosystem taxonomy.

## [2026.6.9] - 2026-06-09

### Renamed
- **Project rename**: `scout` was previously `scout-App` / `rWifi`. The Cargo package name, binary name, file paths, registry keys, and docs are now lowercase `scout`. Behavior and features are unchanged.

### Refactored
- **App Blueprint alignment**: Re-architected directory and module tree to standard App layout. Renamed `src/ui/panels.rs` to `src/ui/widgets.rs`. Created `src/backend/` directory, moving the `src/wlan/` files under `src/backend/wlan/`. Renamed the background thread manager `src/worker.rs` to `src/backend/mod.rs` to act as the standard backend manager.

### Changed
- README rewritten in the new register: WiFi scanner feature list, install matrix, CLI flags, configuration, build instructions, license.
- Drop the legacy "r*" and "Local freedom" branding throughout.
- Drop the per-repo `rApps` umbrella and `build_all.ps1` from this repo; build orchestration lives in [`toolkit`](https://github.com/local76/toolkit).

## [3.1.0] - 2026-06-08

### Changed
- Renamed project back from `scout-App` to `scout` (crate name: `scout`, binary name: `scout`).
- Split monolithic `src/wlan.rs` (1276 lines) into modular `src/wlan/` files (all under 500 lines).
- Split monolithic `src/main.rs` (2025 lines) into `src/app/`, `src/ui/`, and `src/win32.rs` submodules (all under 500 lines).
- Fixed the selection highlight bug where single mouse clicks on App list items or buttons incorrectly triggered full-line selection and clipboard copy.
- Introduced a drag threshold check to prevent mouse clicks from starting text selection unless a drag occurs.

## [3.0.1] - 2026-06-06
### Added
- Added author and maintainer metadata for packaging.

## [3.0.0] - 2026-06-06
### Changed
- Renamed organization to `local76`.
- Renamed executable from `rtem` to `scout`.
- Reorganized directory structure to group packaging files inside `dist/packages/`.