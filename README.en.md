# meatshell-custom

[简体中文](./README.md) | **English**

A lightweight, low-memory SSH / terminal client inspired by FinalShell, but
written entirely in **Rust + [Slint](https://slint.dev)**. The goal is to keep
FinalShell's core experience (resource-monitor sidebar, session management,
tabbed terminals) while cutting memory use from the 400 MB+ of a JVM app down to
the tens-of-MB range of a native binary.

## Custom Version Notes

This branch is based on the original [meatshell](https://github.com/jeff141/meatshell) project, with custom feature enhancements, interaction improvements, and bug fixes. The rest of this README is kept as close to the upstream version as possible.

### Changes and Improvements

The following are additions, interaction improvements, and fixes in this branch compared with `upstream/main`:

- Added the Jiangnan theme and expanded theme tokens; SFTP zebra rows, hover/selected states, and destructive actions now follow the active theme.
- Quick Connect, Resource Status, and Quick Commands use one four-edge docking system. Panels on the same edge share a dock group and can be switched or collapsed through the edge icons, with consistent widths and splitters.
- Quick Commands can be shown as an independent dock, with persistent settings, command groups, and broadcast execution; the Welcome and Resource Status panels can also be docked independently.
- Added remote NVIDIA multi-GPU VRAM monitoring (used / total), a System Information window, bilingual labels, and visibility-aware resource sampling that pauses while the panel is hidden.
- Added X11 forwarding for SSH connections; local PowerShell and CMD sessions use dedicated startup handling and disable controls that do not apply to local shells.
- Added optional terminal output highlighting with Log / DevOps presets and custom rules, plus Windows / macOS renderer selection.
- Added preferences for terminal `Ctrl+C` behavior, selection auto-copy, automatic cursor color, cursor opacity, and the unfocused cursor appearance.
- Refined terminal cursor behavior: typing or navigation keys keep it visible and restart the blink cycle; selection, tmux, and wide-character positioning are handled more consistently.
- Session tabs support same-group drag reordering, horizontal wheel browsing, a draggable tab scrollbar, a title-width cap, and an always-available new-tab page.
- Added SFTP keyboard operations (arrow keys, Backspace, Enter, and Delete), opening files from the path field, delete confirmation with Esc cancellation, and rejection of invalid paths in history.
- SFTP supports Windows-style multi-selection, batch download / move / delete, archive download, visual move-target selection, and directory-tree context menus.
- SFTP file lists can sort by name, size, type, modified time, permissions, and user/group, and retain the active sort and selection across navigation, back/forward, refresh, and reconnect.
- Each session keeps up to 50 SFTP path-history entries and restores directory-tree expansion, selected entries, and scroll positions; the session directory cache keeps up to 64 recent directories.
- Large SFTP directories use paging, virtualized lists, and request coalescing; directory trees support `Load all` and `Collapse` without blocking repeated navigation.
- Added `Files` and `Port Forwarding` SFTP tabs with runtime local `-L` and dynamic `-D` SOCKS5 forwarding, including start, stop, failure, and cleanup states.
- Side-docked SFTP toolbars adapt by shrinking and clipping controls in priority order; the path field, parent/refresh controls, and collapse button remain correctly positioned, and all toolbar buttons have hover tooltips.
- Added tmux `cd` path following, optional session-synchronized uploads, and bilingual SFTP file-type and interface text.
- New / edit session and SFTP dialogs support Esc-to-close and drag repositioning; text fields follow the caret and show the end of long paths by default.
- Bounded terminal scrollback and emoji image caches to prevent long-running sessions or high-volume output from retaining unbounded memory.

## Screenshots

<p align="center">
  <img src="docs/screenshots/01-welcome-en.png" alt="Welcome / session management" width="800"><br>
  <em>Welcome page: session management + local resource monitor sidebar</em>
</p>

<p align="center">
  <img src="docs/screenshots/02-terminal-htop.png" alt="Terminal + SFTP" width="800"><br>
  <em>Tabbed terminal (full-screen btop) + SFTP file browser + remote resource monitoring</em>
</p>

## Download & install

Every `v*` tag triggers a GitHub Actions build that produces native binaries for
**Windows / Linux / macOS**, published on the
[Releases](https://github.com/jeff141/meatshell/releases) page.

### Windows

Download `meatshell-*-windows-x86_64.zip`, unzip, and run `meatshell.exe`.

### Linux

```bash
tar -xzf meatshell-*-linux-x86_64.tar.gz
cd meatshell-*-linux-x86_64
./meatshell                                  # run it directly
# Optional: install the app icon + launcher entry (shows the icon in the dock /
# app list — no argument needed, it finds the binary next to the script)
chmod +x install-linux.sh && ./install-linux.sh
```

> Requires glibc ≥ 2.35 (Ubuntu 22.04+ / Debian 12+). On Wayland you may need to
> log out/in once after installing the icon.

Building from source with `cargo run` on Linux Mint / Ubuntu / Debian requires
the Slint/winit/rfd system development packages:

```bash
sudo apt update
sudo apt install -y --no-install-recommends \
  build-essential pkg-config cmake \
  libfontconfig1-dev libfreetype6-dev \
  libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev \
  libgl1-mesa-dev libegl1-mesa-dev libgtk-3-dev \
  libudev-dev
```

### macOS

The download is a `.zip` containing the `meatshell.app` bundle:

```bash
# Unzip (aarch64 = Apple Silicon, x86_64 = Intel)
unzip meatshell-*-macos-*.zip
# Move it to Applications (optional — it also runs in place)
mv meatshell.app /Applications/
# Clear the quarantine flag, otherwise macOS says "meatshell is damaged and can't be opened"
xattr -dr com.apple.quarantine /Applications/meatshell.app
# Open it (or double-click in Finder)
open /Applications/meatshell.app
```

> If you didn't move it to `/Applications`, point both paths above at wherever the `.app` actually is (e.g. `~/Downloads/meatshell.app`).

> To build from source, see [Running](#running) below.

## Features

### Done

- [x] FinalShell-style UI with dark / light / follow-system themes
- [x] Local + remote resource monitoring (CPU / memory / swap / network / disk)
- [x] Remote process monitor (CPU-sorted table with PID copy and permission-aware termination)
- [x] Full VT/ANSI terminal emulation (btop / htop / vim render correctly)
- [x] Color emoji, including skin tones, flags, and ZWJ sequences
- [x] Tabs (welcome page + multiple sessions)
- [x] Session management: create / edit / delete / groups, local JSON, export / import
  - Config location: `%APPDATA%/meatshell/sessions.json` (Windows)
    / `~/.config/meatshell/sessions.json` (Linux)
    / `~/Library/Application Support/meatshell/sessions.json` (macOS)
- [x] SSH (`russh`, pure Rust): password / private key / encrypted key (passphrase)
- [x] SFTP browser + upload / download (drag-and-drop) + in-terminal ZMODEM (`sz`) receive
- [x] SSH port forwarding / tunnels: local -L / remote -R / dynamic -D (SOCKS5)
- [x] Quick commands + command box (broadcast to all sessions) + command history
- [x] Serial / Telnet sessions
- [x] Outbound proxy (SOCKS5 / HTTP)
- [x] Import `~/.ssh/config`
- [x] Session passwords encrypted at rest (ChaCha20-Poly1305)
- [x] Known-hosts (`known_hosts`) verification + first-connect confirmation
- [x] Split panes for tabbed terminals

Color emoji graphics are provided by [Twemoji](https://github.com/jdecked/twemoji)
under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/). See
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for the full attribution.

### Planned

- [ ] Store session passwords in the OS keychain

## Tech stack

| Module        | Choice                                                            |
| ------------- | ----------------------------------------------------------------- |
| UI            | [Slint](https://slint.dev) (compiled pure Rust, no GC)            |
| Async runtime | [`tokio`](https://tokio.rs)                                       |
| SSH protocol  | [`russh`](https://crates.io/crates/russh) (no libssh dependency)  |
| System metrics| [`sysinfo`](https://crates.io/crates/sysinfo)                     |
| Serialization | `serde` + `serde_json`                                            |
| Logging       | `tracing` + `tracing-subscriber`                                  |

## Running

```bash
cargo run --release
```

On first launch an empty session store is created at
`%APPDATA%/meatshell/sessions.json`. Click **"＋ New Session"** in the top-right
to add your first server.

## Project layout

```
meatshell/
├── Cargo.toml
├── build.rs                 # Slint compiler entry point
├── ui/
│   ├── app.slint            # top-level window
│   ├── theme.slint          # design tokens
│   ├── widgets.slint        # reusable buttons / inputs / sparkline
│   ├── sidebar.slint        # left-hand system monitor panel
│   ├── tabs.slint           # top tab bar
│   ├── welcome.slint        # welcome page / quick connect
│   ├── session_dialog.slint # new / edit session dialog
│   └── terminal_view.slint  # terminal view (v0.1 line-buffered)
└── src/
    ├── main.rs
    ├── app.rs               # UI ↔ backend bridge
    ├── config.rs            # session JSON persistence
    ├── system.rs            # CPU / memory / network sampling
    └── ssh.rs               # SSH session worker
```

## Development notes

- Slint widgets use a strict layout DSL; after editing a `.slint` file,
  `cargo check` is the fastest feedback loop.
- The application event loop is single-threaded (required by Slint); all
  cross-thread UI updates go through `slint::invoke_from_event_loop` callbacks.
- SSH / SFTP share the `known_hosts` verification path: first contact asks for
  trust and remembers the host key, while later key changes prompt again.

## Release

Do not bump `Cargo.toml` by hand and then create a tag. Use the release helper
so the tag points at a commit that already contains the matching Cargo version:

```powershell
.\scripts\release.ps1 v0.6.0 -Push
```

You can also use a custom tag / release name. If the tag cannot be used to
derive the Cargo version, pass `-Version` explicitly:

```powershell
.\scripts\release.ps1 my-build "v0.5.7 custom" -Version 0.5.7-custom.1 -Push
```

The script updates `Cargo.toml` / `Cargo.lock`, runs `cargo check --locked`,
verifies `meatshell --version`, commits `Release v0.6.0`, creates an annotated
tag, and pushes the current branch plus the tag. See
[docs/release.md](docs/release.md) for details.

## Related Groups

<p align="center">
  <img src="docs/QR/QQ_Group_QR_Code.jpg" alt="QQ group QR code" width="300"><br>
  <em>Scan the QR code to join QQ groups to exchange user experiences, provide feedback, or get the latest updates</em>
</p>

## License

Dual-licensed under MIT OR Apache-2.0.

## License

MIT OR Apache-2.0 dual license.
