<div align=center>

[![Downloads](https://img.shields.io/github/downloads/__REPO__/v__VERSION__/total?style=flat-square&logo=github)](https://github.com/__REPO__/releases/tag/v__VERSION__)

</div>

### ⬇️ Downloads

| OS / Arch | x86_64 | ARM64 |
|-----------|--------|-------|
| **Windows** | [![x64](https://img.shields.io/badge/x86_64-0078D4.svg?logo=windows)](__BASE_URL__/htop-rust-for-win-x86_64-v__VERSION__.exe) | [![ARM64](https://img.shields.io/badge/ARM64-0099CC.svg?logo=windows)](__BASE_URL__/htop-rust-for-win-arm64-v__VERSION__.exe) |

> **Build Toolchain**: Rust stable + MSVC. Requires **Windows 10+** (Windows 7 dropped since Rust 1.77+).

### 📥 Quick Install

**Cargo (build from source):**
```bash
cargo install htop-rust-for-win@__VERSION__
```

**Scoop:**
```powershell
scoop bucket add vincentzyu https://github.com/VincentZyuApps/scoop-bucket
scoop install htop-rust-for-win
```

### 🚀 Usage

```bash
# 🎯 Default dashboard
htop-rust-for-win

# ⏱ Custom refresh interval
htop-rust-for-win -t 500

# 📖 Show help
htop-rust-for-win --help
```

### Commit Log

__COMMIT_LOG__
