# 🍉 Watermelon

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)
![GTK4](https://img.shields.io/badge/GTK-4.0-green)
![License](https://img.shields.io/badge/License-MIT-blue)
![Platform](https://img.shields.io/badge/Platform-Linux-lightgrey)

**Fresh. Native. Uncompromising.**

A modern, high-performance note-taking experience built exclusively for the Linux desktop.  
**No Electron. No Cloud. Just Notes.**

---

## ✨ Features

*   **🚀 Native Performance**: Built with **Rust** and **GTK4** for instant startup and zero lag.
*   **📝 Rich Markdown**:
    *   **Formatting**: Write naturally with **Bold**, *Italic*, Lists, and Links.
    *   **Tasks**: Interactive checkboxes (`[ ]` -> `[x]`) that toggle with a click.
    *   **Shortcuts**: Familiar keybindings for rapid editing.
*   **📂 Powerful Organization**:
    *   **Folders**: Create custom folders to structure your thoughts.
    *   **Drag & Drop**: Intuitively move notes between folders.
    *   **Search**: Find any note instantly with the sidebar search.
*   **🔒 Privacy Focused**:
    *   **Local First**: All data is stored in a local **SQLite** database.
    *   **Offline**: Works perfectly without an internet connection.
*   **🎨 Beautiful UI**: Designed with **Libadwaita** to look right at home on GNOME.

## 📥 Installation

### Option 1: Run from Source (Recommended)

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/aashiqumar/Watermelon.git
    cd Watermelon
    ```

2.  **Install dependencies** (Fedora):
    ```bash
    sudo dnf install gtk4-devel libadwaita-devel gcc
    ```
    *(For Ubuntu/Debian: `sudo apt install libgtk-4-dev libadwaita-1-dev build-essential`)*

3.  **Run the app**:
    ```bash
    cargo run
    ```

## ⌨️ Shortcuts

Watermelon is designed for keyboard efficiency.

| Shortcut | Action |
| :--- | :--- |
| **Ctrl + B** | Toggle **Bold** formatting |
| **Ctrl + I** | Toggle *Italic* formatting |
| **Ctrl + N** | Create a **New Note** |
| **Double Click** | Rename a folder in the sidebar |
| **Click** | Toggle a checkbox `[ ]` / `[x]` |

## 🛠️ Building for Release

To create an optimized binary for daily use:

```bash
cargo build --release
```
The binary will be located at `target/release/watermelon`.

## 🗺️ Roadmap

*   [ ] **Tag System**: Flexible filtering with #tags.
*   [ ] **Export**: PDF and HTML export options.
*   [ ] **Images**: Drag & drop image support.
*   [ ] **Sync**: Optional encrypted cloud sync.

## 📄 License

This project is licensed under the MIT License.

## 👨‍💻 Credits

**Created by [Aashiq Umar](https://github.com/aashiqumar)**

---
<div align="center">
  <sub>Built with ❤️ and 🦀 in Sri Lanka.</sub>
</div>
