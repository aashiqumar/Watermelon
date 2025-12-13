# Watermelon Development Roadmap

## 1. Development Phases & Milestones

### Phase 1: Prototyping (Weeks 1-2)
**Goal**: Get a window on screen with the basic layout.
*   [ ] Setup GTK4 + Relm4 project structure.
*   [ ] Implement the 3-pane layout (Sidebar, List, Editor).
*   [ ] Basic navigation (Clicking a note item updates the editor view).
*   [ ] **Milestone**: "Skeleton Alpha" - A clickable UI with dummy data.

### Phase 2: Core Logic Implementation (Weeks 3-4)
**Goal**: Make it functional.
*   [ ] Implement SQLite database & `NoteRepository`.
*   [ ] Connect UI to Data (CRUD operations).
*   [ ] Implement Autosave logic.
*   [ ] **Milestone**: "Functional Beta" - You can write notes and they persist.

### Phase 3: Editor & Search (Weeks 5-6)
**Goal**: Make it powerful.
*   [ ] Integrate `GtkSourceView` with Markdown highlighting.
*   [ ] Implement Search (Fuzzy + Database).
*   [ ] Add Tagging system.
*   [ ] **Milestone**: "Feature Complete" - All core features working.

### Phase 4: Polish & Packaging (Weeks 7-8)
**Goal**: Make it shippable.
*   [ ] Theming (Watermelon colors, Dark mode).
*   [ ] Animations & Transitions.
*   [ ] Packaging for Fedora (RPM, Flatpak).
*   [ ] **Milestone**: "v1.0 Release".

## 2. Folder Structure

We will refine the structure to support the scale:

```
Watermelon/
├── .github/            # CI/CD workflows
├── build-aux/          # Flatpak manifests, RPM specs
├── data/               # Desktop files, icons, GSettings schemas
├── po/                 # Translations (i18n)
├── src/
│   ├── components/     # UI Widgets (Relm4 Components)
│   ├── core/           # Business Logic (Services)
│   ├── db/             # Database Access (Repositories)
│   ├── models/         # Data Structures
│   └── utils/          # Helpers
├── tests/              # Integration tests
└── Cargo.toml
```

## 3. Git Branching Strategy

We will use **GitHub Flow** (Simple & Fast).
*   `main`: Always deployable. Represents the current stable state.
*   `feature/xyz`: Create a new branch for every feature/fix.
*   **PR Process**:
    1.  Create branch.
    2.  Push code.
    3.  CI runs tests.
    4.  Merge to `main` via Pull Request.

## 4. Testing Plan

### Unit Tests
*   **Scope**: `src/core`, `src/models`, `src/utils`.
*   **Tool**: Standard `cargo test`.
*   **Coverage**: Aim for >80% on business logic (e.g., Markdown parsing, Search algorithms).

### UI / Integration Tests
*   **Scope**: Database interactions, UI flows.
*   **Tool**: `gtk-test` (if applicable) or headless integration tests for the DB layer.
*   **Manual**: "Dogfooding" - Use Watermelon to build Watermelon.

## 5. Continuous Integration (CI)

**Platform**: GitHub Actions.

**Workflow (`.github/workflows/ci.yml`)**:
1.  **Check**: `cargo check`
2.  **Lint**: `cargo clippy -- -D warnings`
3.  **Format**: `cargo fmt -- --check`
4.  **Test**: `cargo test`
5.  **Build**: `cargo build --release`

## 6. Packaging for Fedora

### RPM (Native Package)
*   **Spec File**: `watermelon.spec`
*   **Dependencies**: `gtk4`, `libadwaita`, `sqlite`.
*   **Build**: Use `fedpkg` or standard `rpmbuild`.
*   **Repo**: Publish to a COPR repository (`@user/watermelon`).

### Flatpak (Universal Linux)
*   **Manifest**: `org.example.Watermelon.json` (or YAML).
*   **Runtime**: `org.gnome.Platform` // `45`.
*   **SDK**: `org.gnome.Sdk` // `45`.
*   **Distribution**: Flathub.

## 7. Distribution Plan

1.  **Alpha**: Release `.tar.gz` binaries on GitHub Releases.
2.  **Beta**: Launch Fedora COPR repository for easy updates (`dnf install watermelon`).
3.  **Stable**: Submit to Flathub for broad Linux support.
