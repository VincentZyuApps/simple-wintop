![simple-wintop](https://socialify.git.ci/VincentZyuApps/simple-wintop/image?custom_description=%F0%9F%93%8A%F0%9F%A6%80+htop-inspired+Windows+system+monitor+%E2%80%94+CPU+bars%2C+Memory+%26+Swap%2C+Tasks%2C+Uptime+in+a+colorful+TUI+%28Rust+%2B+ratatui%29&description=1&forks=1&issues=1&language=1&logo=https%3A%2F%2Favatars.githubusercontent.com%2Fu%2F250448479%3Fs%3D200%26v%3D4&name=1&owner=1&pulls=1&stargazers=1&theme=Auto)

# 🖥️ simple-wintop 🦀

[![Windows x64 | ARM64](https://img.shields.io/badge/Windows-x64_|_ARM64-0078D4?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/VincentZyuApps/simple-wintop/releases)

> 🌟 A lightweight, real-time **system monitor dashboard** for Windows — inspired by [htop](https://github.com/htop-dev/htop) 🚀
> Purely the **top meters section** — CPU bars 🧠, Memory & Swap usage 🫧, Tasks counter 🔢, and Uptime ⏱ — all in a beautiful multi‑color TUI 🎨.

> **[📖 English](readme.md)**
> **[📖 简体中文](readme.zh-cn.md)**

## ✨ Highlights

- 📊 **CPU meters** — per‑core usage bars with green→yellow→red gradient for low→high load 🟢🟡🔴
- 🧠 **Memory & Swap** — real‑time usage bars with multi‑color segments (green → blue → yellow → red) 🟦🟨
- 🔢 **Tasks counter** — total processes and running count at a glance 🔄
- ⏳ **Uptime** — system uptime since last boot 🕒
- ⌨️ **Keyboard shortcuts** — `q` / `Esc` to quit, instant and responsive 🎯
- 🎨 **Colorful TUI** — powered by [ratatui](https://github.com/ratatui-org/ratatui) + [crossterm](https://github.com/crossterm-rs/crossterm) ✨
- 🪟 **Windows only** — built natively for x86_64 and ARM64 🏗️

## 🖼️ Screenshot

![screenshot](docs/screenshot.png)

## 🚀 Quick Start

```bash
# 📖 Show help
simple-wintop --help
# 🎯 Default dashboard — CPU, Memory, Swap, Tasks, Uptime
simple-wintop
# ⏱️ Set custom refresh interval (in milliseconds)
simple-wintop -t 500
# ℹ️ Show version
simple-wintop --version
```

## ⚙️ Common Flags

| Flag | Description |
|------|-------------|
| `-t, --interval <ms>` | Refresh interval in milliseconds (default: 1000) |
| `--help` | Print help message |
| `--version` | Print version info |

### ⌨️ Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q` / `Q` / `Esc` | 🚪 Quit |

## 🔧 Build from Source

```bash
# 🦀 Build with Cargo
cargo build --release
```

GitHub Actions automatically builds **Windows x86_64** and **Windows ARM64** binaries.

## 📄 License

📝 MIT — see [LICENSE](LICENSE)
