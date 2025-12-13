# Watermelon v1 Specification

## 1. Feature List

### Core Features (v1)
*   **Note Management**: Create, Read, Update, and Delete (CRUD) notes.
*   **Rich Text Editing**: Basic formatting (Bold, Italic, Underline, Strikethrough, Lists, Headings).
*   **Sidebar Navigation**: A clean, resizeable sidebar listing all notes.
*   **Search**: Instant search across note titles and content.
*   **Folders/Categories**: Organize notes into folders or use tags.
*   **Auto-save**: Changes are saved automatically in real-time.
*   **Dark/Light Mode**: Seamless integration with the system theme (Fedora/GNOME).
*   **Offline First**: All data stored locally.

### Optional Future Features
*   **Cloud Sync**: Sync with Nextcloud, Google Drive, or a custom backend.
*   **Markdown Support**: Full Markdown syntax highlighting and export.
*   **Attachments**: Drag and drop images and files into notes.
*   **Versioning**: History of note changes.
*   **Export**: Export notes to PDF, HTML, or Markdown.
*   **Plugins**: Extension system for community features.

## 2. Tech Stack Recommendation

**Recommended Stack: GTK4 + Libadwaita + Rust**

### Comparison

| Feature | **GTK4 + Rust (Recommended)** | **Electron** | **Qt (C++/Python)** |
| :--- | :--- | :--- | :--- |
| **Performance** | 🚀 **Excellent**. Native binary, low memory footprint. | 🐢 **Heavy**. Runs a full browser instance per app. | 🚀 **Good**. Native C++, but Python bindings add overhead. |
| **Look & Feel** | 🎨 **Native**. Perfect integration with Fedora/GNOME (Libadwaita). | 🖌️ **Custom**. Can look like anything, but feels "web-like". | 🖥️ **Native-ish**. Good integration, especially in KDE, but distinct style. |
| **Dev Experience** | 🛡️ **Safe**. Rust prevents memory bugs. Strict compiler. | ⚡ **Fast**. Hot-reload, web dev skills transfer directly. | ⚙️ **Complex**. C++ is hard. Python is easier but less performant. |
| **Cross-Platform** | ⚠️ **Moderate**. Works on Linux best. Windows/macOS possible but requires effort. | 🌍 **Excellent**. Write once, run everywhere perfectly. | 🌍 **Great**. Strong cross-platform history. |
| **Binary Size** | 📦 **Tiny**. A few MBs. | 🐘 **Huge**. 100MB+ minimal size. | 📦 **Medium**. Depends on bundled libraries. |

### Why GTK4 + Rust?
For a **Fedora Linux** targeted app, **GTK4 + Libadwaita** is the gold standard. It provides the "Apple-like" smoothness and consistency with the OS that users expect on modern Linux desktops. Rust ensures the app is crash-free and blazingly fast, fitting the "Minimal" and "Clean" philosophy.

## 3. Brand Design Philosophy

**"Fresh. Clean. Friendly."**

*   **Minimalism**: UI elements should only appear when needed. Content comes first.
*   **Whitespace**: Generous use of padding and margins to let the content breathe.
*   **Typography**: Use system fonts (Cantarell/Inter) with careful attention to weight and hierarchy.
*   **Smoothness**: Animations for hover states, transitions between notes, and opening/closing panels.
*   **Rounded**: Use the Libadwaita rounded corners aesthetic for windows and buttons.
*   **Color**: A soft, pastel-inspired palette (Watermelon theme) for accents, keeping the base neutral (White/Dark Grey).

## 4. Project Architecture

### Folder Structure
```
Watermelon/
├── Cargo.toml          # Rust dependencies
├── docs/               # Documentation and Specs
│   └── WATERMELON_V1_SPEC.md
├── src/
│   ├── main.rs         # Entry point
│   ├── app.rs          # Main Application Logic / Window setup
│   ├── components/     # UI Components
│   │   ├── mod.rs
│   │   ├── sidebar.rs  # Note list and navigation
│   │   ├── editor.rs   # Rich text editor area
│   │   └── toolbar.rs  # Action buttons
│   ├── models/         # Data Structures
│   │   ├── mod.rs
│   │   └── note.rs     # Note struct and logic
│   └── utils/          # Helper functions
│       └── mod.rs
└── assets/             # Icons, CSS, resources
```

### Architecture Overview (The Elm Architecture / MVC)
*   **Model**: The state of the application (List of notes, Current selected note, Search query).
*   **View**: The GTK4 widgets that render the state.
*   **Update**: Messages (Enums) that modify the state (e.g., `NoteSelected(id)`, `ContentChanged(text)`).
*   **Relm4 / gtk-rs**: We will likely use **Relm4** (an idiomatic Rust GUI library based on the Elm architecture) or raw **gtk4-rs** depending on preference for reactivity vs control. Relm4 is recommended for "Fresh" codebases.
