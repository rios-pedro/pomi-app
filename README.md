# Pomi 🍅

A minimalist Pomodoro timer for macOS that lives in the menu bar, distraction-free.

## Stack

- [Tauri 2](https://tauri.app/) — Rust + native WebView
- React + TypeScript
- `tauri-plugin-positioner` — anchors the window near the tray icon
- `tauri-plugin-notification` — native macOS notifications

## Features

- Menu bar icon showing the remaining time in real time
- No Dock icon, no Cmd+Tab entry (runs as `Accessory`)
- Quick presets (5, 15, 25, 45 min) + customizable slider
- Visual progress ring around the timer that changes color in the last 20% of the time
- Popover-style floating window anchored below the tray icon, with rounded corners (transparent window)
- Auto-hides when clicking outside or switching apps (hide on blur)
- Native notification when time runs out

## Running locally

```bash
npm install
npm run tauri dev
```

## Production build

```bash
npm run tauri build
```

The generated `.app` will be at `src-tauri/target/release/bundle/macos/pomi.app`.

> **Note:** notifications only work reliably in the built `.app` (`tauri build`), not in dev mode (`tauri dev`) — macOS doesn't register dev server apps in the Notification Center.

## Changing the app icon

```bash
npm run tauri icon path/to/icon-1024x1024.png
```

This automatically generates every required size (`.icns`, `.ico`, PNGs) inside `src-tauri/icons/`.

## Project structure

src/ → React/TS frontend (UI, timer logic)
src-tauri/src/ → Rust backend (tray, commands, window, auto-hide)
src-tauri/capabilities/ → Tauri permissions (notification, positioner, etc.)
src-tauri/icons/ → app icons in all sizes

## License

MIT
