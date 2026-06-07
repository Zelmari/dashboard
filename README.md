# dashboard

A terminal system monitor written in Rust.

## What it does

Run it from anywhere:

```
dashboard
```

It opens a full-screen terminal UI showing CPU usage across all cores, memory
pressure, and a live process table sorted by CPU. Processes that spike or drop
significantly get surfaced in a separate sidebar so you can see what's changing
at a glance.

Refreshes every 5 seconds by default. Low overhead, it uses targeted OS calls
rather than a full system scan on each tick.

## Installing

You'll need Rust installed. Then from inside the project directory:

```
chmod +x install.sh
./install.sh
```

This builds a release binary and copies it to `/usr/local/bin/dashboard` so
you can run it from anywhere. If `/usr/local/bin` needs sudo, the script
handles that automatically.

To install somewhere else:

```
./install.sh ~/.local/bin
```

To uninstall:

```
rm $(which dashboard)
```

## Navigation

The process table is the only interactive part. Everything else updates
automatically.

| Key          | Action            |
| ------------ | ----------------- |
| ↑ / ↓        | move up / down    |
| Ctrl+N       | move down         |
| Ctrl+P       | move up           |
| PgDn / PgUp  | jump 10 rows      |
| Home / End   | top / bottom      |
| Ctrl+C       | quit              |

## Options

```
dashboard --refresh-ms 2000
```

Sets the refresh interval in milliseconds. Default is 5000.

## What's on screen

**CPU panel** — a one-line sparkline showing recent history, then an overall
usage bar, then a two-column grid of per-core bars. Load averages are shown
bottom-right. All bars use block characters so they render cleanly in any
terminal font.

**Process table** — sorted by CPU descending. Shows PID, name, CPU%, memory,
status, and user. Process names fade from bright to dim as you go down the
list, so the busy ones stand out immediately.

**Memory panel** — total, used, and available RAM with accurate figures.
Uses `total - available` rather than the system's reported used value, which
on macOS inflates with file cache.

**Active sidebar** — processes whose CPU usage changed significantly since the
last tick. Rising ones are shown in red, falling in green.

## How it is structured

Five modules, each with one job.

- `system.rs` — all OS communication lives here. Wraps sysinfo and returns
  typed snapshots. Nothing else in the app imports sysinfo directly, so
  swapping the backend only touches this file.
- `app.rs` — mutable UI state between frames. Selected row, sort column,
  tick counter, and the refresh logic.
- `theme.rs` — every colour and style constant in one place. Changing the
  palette means changing this file only.
- `ui/` — one file per panel. `mod.rs` composes them into the layout.
  Adding a new panel means creating a new file and carving out a rect in
  `mod.rs`.
- `main.rs` — terminal setup, the event loop, and cleanup. Installs a panic
  hook so the terminal is always restored even on a crash.

## Dependencies

- `ratatui` — terminal UI layout and widgets
- `crossterm` — cross-platform raw mode and input events
- `sysinfo` — CPU, memory, and process data
- `human_bytes` — human-readable byte formatting
- `anyhow` — ergonomic error propagation
