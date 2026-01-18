# 🕵️ Lazy Monitor

> **Quantify your digital life without lifting a finger.**
> A minimalist time tracker built with Rust that automatically quantifies your attention distribution.

![Rust](https://img.shields.io/badge/Made_with-Rust-orange?style=flat-square)
![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)

## 📖 Introduction

**Lazy Monitor** is a lightweight, background process that automatically tracks which application you are currently using. It generates real-time reports to help you understand where your time actually goes.

Unlike other time trackers, it requires **zero manual input**. Just run it, forget it, and check the dashboard when you feel guilty.

## ✨ Features

* **⚡ Zero Latency:** Built with Rust, consuming negligible CPU & RAM.
* **📊 Visual Dashboard:** Auto-generates a `report.html` with interactive charts (powered by Chart.js).
* **📝 Persistent Logging:** detailed usage logs are saved to `log.csv` for further analysis (e.g., Excel/Pandas).
* **🛡 Privacy First:** All data is stored locally. Nothing is uploaded to the cloud.

## 🚀 Quick Start

### Option 1: Download Binary (Recommended)
Go to the [Releases](https://github.com/你的用户名/lazy-monitor/releases) page and download the latest executable.

### Option 2: Build from Source
If you have Rust installed:

```bash
git clone [https://github.com/YOUR_USERNAME/lazy-monitor.git](https://github.com/YOUR_USERNAME/lazy-monitor.git)
cd lazy-monitor
cargo run --release

## 🎮 Usage

1. Run the executable `lazy-monitor.exe`.
2. Keep the terminal window open (or minimize it).
3. Use your computer as usual.
4. Open `report.html` in your folder to see the **Real-time Pie Chart**.
5. Press `Ctrl + C` in the terminal to stop monitoring.

## 🛠 Tech Stack (Under the Hood)

This project demonstrates core Systems Engineering concepts:

* **Polling & Aggregation:** Efficiently monitoring OS window handles using `active-win-pos-rs`.
* **Stream Processing:** Real-time data processing loop with noise filtering.
* **Data Persistence:** Structured CSV logging using standard I/O.
* **Visualization:** Dynamic HTML injection for frontend reporting.

## 📊 Example Output

**Terminal:**

```text
[2026-01-18 21:40:05] Chrome -> 120s
[2026-01-18 21:42:15] VS Code -> 850s

```

**Dashboard:**
*(Here you can put a screenshot of your report.html later)*

## 📄 License

This project is licensed under the MIT License.
