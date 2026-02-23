# TaurApp AI Coding Instructions

## Project Overview
TaurApp is a WhatsApp desktop client built with Tauri v1 + Rust. The frontend is minimal (just redirects to web.whatsapp.com via [dist/script.js](../dist/script.js)), while the Rust backend handles system tray integration and window management.

## Architecture & Key Components

### Rust Backend Structure
- **[src-tauri/src/main.rs](../src-tauri/src/main.rs)**: Entry point. Sets up logging, system tray, and event handlers.
- **[src-tauri/src/lib.rs](../src-tauri/src/lib.rs)**: Module exports (logging, tray, window).
- **[src-tauri/src/tray.rs](../src-tauri/src/tray.rs)**: System tray menu logic with show/hide/quit actions. Dynamic menu updates based on window visibility state.
- **[src-tauri/src/window.rs](../src-tauri/src/window.rs)**: Intercepts close events to minimize to tray instead of exiting.
- **[src-tauri/src/logging.rs](../src-tauri/src/logging.rs)**: Fern+chrono logging with timestamp formatting.

### Frontend Structure
The app doesn't build a traditional frontend—[dist/index.html](../dist/index.html) displays "Loading..." then [dist/script.js](../dist/script.js) redirects to `https://web.whatsapp.com`.

### Configuration Files
- **[src-tauri/Cargo.toml](../src-tauri/Cargo.toml)**: Rust dependencies. Uses Tauri 1.2.x with `api-all` and `system-tray` features.
- **[src-tauri/tauri.conf.json](../src-tauri/tauri.conf.json)**: Tauri config. Note `devPath` and `distDir` both point to `../dist`.
- **[package.json](../package.json)**: Minimal npm setup for Tauri CLI only.

## Critical Patterns & Conventions

### Licensing & Copyright
**All Rust source files must include the MPL-2.0 header:**
```rust
// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
```

### Rust Patterns
- **Type-safe identifiers**: Use enums with `From<String>` and `Into<String>` traits (see `TrayIdentifier` in [tray.rs](../src-tauri/src/tray.rs)).
- **Error handling**: Use `.expect()` with descriptive messages for critical operations (e.g., `app.get_window("main").expect("Could not get the main window.")`).
- **Module visibility**: Use `pub` for public APIs, `pub(crate)` for internal cross-module access (see `get_system_tray_menu` in [tray.rs](../src-tauri/src/tray.rs)).
- **Logging levels**: Debug for flow tracking, info for user actions, warn for edge cases, error for failures.

### Window Management Pattern
The app **hides** windows instead of closing them:
1. Close button intercepted in [window.rs](../src-tauri/src/window.rs) via `api.prevent_close()`.
2. Window hidden, system tray menu updated to show "Show" option.
3. User can restore from tray or quit completely via tray menu.

### System Tray Behavior
- Menu is **dynamic**: changes between "Hide" and "Show" based on `window.is_visible()`.
- Tray menu updates happen in two places: `window.rs` (on close) and `tray.rs` (on menu click).
- Icon path: `icons/128x128.png` (see [tauri.conf.json](../src-tauri/tauri.conf.json)).

## Development Workflows

### Running the App
```bash
npm run tauri dev    # Development mode with Rust hot-reload
npm run tauri build  # Production build (creates AppImage, .deb, binary)
```

### Build Process
1. Tauri looks for frontend in `dist/` (no build step needed—files are committed).
2. Rust backend compiled by Cargo via `tauri build`.
3. CI/CD: [.github/workflows/build.linux.yaml](../.github/workflows/build.linux.yaml) builds for Linux on push/PR. Artifacts: binary, AppImage, .deb.

### Versioning
Version must be updated in **two places**:
- [src-tauri/Cargo.toml](../src-tauri/Cargo.toml) → `version = "0.1.x"`
- [src-tauri/tauri.conf.json](../src-tauri/tauri.conf.json) → `package.version = "0.1.x-alpha"`

Update [CHANGELOG.md](../CHANGELOG.md) following [Keep a Changelog](https://keepachangelog.com/) format.

### Dependencies
- **Rust**: Install system dependencies for Linux builds: `libdbus-1-dev`, `pkg-config`, `libgtk-3-dev`, `libwebkit2gtk-4.0-dev`, `libayatana-appindicator3-dev`.
- **Node**: Only needed for Tauri CLI (`@tauri-apps/cli`).

## Important Implementation Notes

### Adding New System Tray Items
1. Add variant to `TrayIdentifier` enum in [tray.rs](../src-tauri/src/tray.rs).
2. Implement `From<String>` and `Into<String>` trait conversions.
3. Create `CustomMenuItem` in `get_system_tray_menu()`.
4. Handle action in `handle_tray_event()` match statement.

### Modifying Window Behavior
All window event logic lives in [window.rs](../src-tauri/src/window.rs). The `CloseRequested` event is critical—removing `api.prevent_close()` would break the minimize-to-tray pattern.

### Logging Changes
Logging is initialized once in `main()` via `logging::setup_logger()`. To change log format/level, edit [logging.rs](../src-tauri/src/logging.rs). Current level: `Debug` (shows all logs).

## Testing & Debugging
- **Logs**: Appear in stdout (terminal where `tauri dev` runs).
- **No automated tests**: Project is experimental—manual testing only.
- **Debugging Rust**: Use `log::debug!()` statements liberally (already used throughout).

## Project State & Context
- **Status**: Experimental/alpha. Missing autoupdate, limited features.
- **Target platforms**: Linux (Ubuntu) primary. macOS/Windows configs present but untested in CI.
- **Distribution**: Manual download from GitHub Releases.
