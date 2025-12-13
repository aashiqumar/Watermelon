 # Watermelon Core Logic Architecture

## 1. Architectural Pattern: MVU (Model-View-Update)

For a Rust + GTK4 application, the **MVU (Elm Architecture)** pattern is the gold standard. It provides a clean, unidirectional data flow that is easy to debug and test. We will use **Relm4** as the framework.

### Components
*   **Model**: A pure Rust struct holding the entire state of the component.
*   **Msg (Message)**: An enum representing every possible event (User input, Timer, Database result).
*   **Update**: A pure function `fn update(model, msg) -> Command` that modifies the model and returns commands (side effects).
*   **View**: A macro or function that declaratively renders the UI based on the Model.

### Clean Architecture Layers
1.  **Presentation Layer (Relm4)**: Handles UI events and rendering.
2.  **Application Layer (Services)**: Orchestrates logic (e.g., `NoteService`, `SearchService`).
3.  **Domain Layer (Models)**: Pure data structures (`Note`, `Folder`).
4.  **Infrastructure Layer (Repositories)**: SQLite access, File I/O.

## 2. State Management

The global application state is managed in the root `AppModel`.

```rust
struct AppModel {
    notes: Vec<Note>,           // Cache of loaded notes
    selected_note_id: Option<Uuid>,
    search_query: String,
    sidebar_visible: bool,
    current_mode: AppMode,      // View/Edit
    // ...
}

enum AppMsg {
    SelectNote(Uuid),
    CreateNote,
    DeleteNote(Uuid),
    UpdateContent(String),      // From Editor
    Search(String),
    AutoSaveTick,               // Timer event
}
```

## 3. CRUD & Autosave Logic

### Create
1.  User clicks "New Note".
2.  `AppMsg::CreateNote` is dispatched.
3.  **Update**:
    *   Generate new UUID.
    *   Create `Note` struct with default title "New Note".
    *   Insert into SQLite (Infrastructure).
    *   Add to `AppModel.notes`.
    *   Set `selected_note_id` to new UUID.

### Read (List)
1.  On startup, `AppMsg::Initialize` calls `NoteRepository::get_all()`.
2.  Populate `AppModel.notes`.

### Update (Autosave)
**Behavior**: "Save as you type" with debouncing.
1.  User types in Editor -> `AppMsg::UpdateContent(text)` dispatched on every keystroke (or buffered).
2.  **Update**:
    *   Update `AppModel.notes[selected]`.
    *   Reset/Start a `DebounceTimer` (e.g., 500ms).
3.  **Timer Fires** -> `AppMsg::AutoSaveTick`.
4.  **Update**:
    *   Check if dirty.
    *   Call `NoteRepository::save(note)` to write to SQLite.
    *   Update `updated_at` timestamp.

### Delete
1.  User clicks "Delete".
2.  `AppMsg::DeleteNote(id)` dispatched.
3.  **Update**:
    *   Set `is_deleted = true` in DB (Soft Delete).
    *   Remove from `AppModel.notes` (or move to Trash folder in UI).

## 4. Rich-Text Editor Architecture

We will use **GtkSourceView 5** (part of GTK4 ecosystem).

*   **Markdown Support**:
    *   GtkSourceView has built-in Markdown syntax highlighting (`LanguageManager`).
    *   We can style headers/bold/italic using a custom `GtkSourceStyleScheme` (Watermelon Theme).
*   **WYSIWYG-ish**:
    *   While the underlying format is Markdown, we can hide syntax characters (like `**bold**`) when not focused, or just use nice styling for headers to make it feel rich.
*   **Clipboard**:
    *   Standard GTK Clipboard handling.
    *   **Paste Image**: Intercept `paste` signal -> Check if image -> Save to `attachments` -> Insert `![img](...)` markdown.

## 5. Search Engine

### Strategy: Hybrid
1.  **In-Memory (Fast)**:
    *   For < 10,000 notes, keeping metadata (Title, Tags, Preview) in memory is instant.
    *   Filter `AppModel.notes` using fuzzy matching (e.g., `sublime_fuzzy` crate).
2.  **Full-Text Search (Deep)**:
    *   For searching *content* of all notes.
    *   Use SQLite **FTS5** module.
    *   Query: `SELECT id FROM notes_fts WHERE content MATCH 'query'`.

## 6. Tag & Folder System

### Folders (Hierarchical)
*   **Logic**: A note belongs to exactly one folder (FK `folder_id`).
*   **UI**: Tree view in Sidebar.
*   **Drag & Drop**: Drag note card -> Drop on Folder -> Update `folder_id`.

### Tags (Flat)
*   **Logic**: Many-to-Many (`note_tags` table).
*   **UI**:
    *   Displayed as pills in Note Card and Editor.
    *   Hashtag parsing: If user types `#idea` in editor, auto-create tag and link it.

## 7. Undo/Redo

*   **Editor Content**: `GtkSourceBuffer` has built-in `can-undo`, `can-redo`, `undo()`, `redo()` methods. We get this for free!
*   **App Actions (Delete/Move)**:
    *   Implement a `Command` trait: `execute()` and `undo()`.
    *   Maintain a `Vec<Box<dyn Command>>` stack.
    *   Example: `DeleteNoteCommand` stores the deleted note data to restore it on undo.

## 8. Event Listeners

In Relm4, event listeners are declarative:

```rust
view! {
    gtk::TextView {
        connect_buffer_changed[sender] => move |_| {
            sender.input(AppMsg::UpdateContent(buffer.text()));
        }
    }
}
```
This decouples the UI from the logic completely.
