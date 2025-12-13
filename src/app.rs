use crate::components::editor::{Editor, EditorMsg};
use crate::components::navigation::{Navigation, NavigationOutput};
use crate::components::sidebar::{Sidebar, SidebarMsg};
use crate::core::note_service::NoteService;
use crate::models::note::Note;
use gtk::prelude::*;
use relm4::prelude::*;
use std::rc::Rc;

pub struct App {
    navigation: Controller<Navigation>,
    sidebar: Controller<Sidebar>,
    editor: Controller<Editor>,
    notes: Vec<Note>,
    note_service: Rc<NoteService>,
    selected_index: Option<usize>,
    current_folder: String,
}

#[derive(Debug)]
pub enum AppMsg {
    SidebarMsg(SidebarMsg),
    EditorMsg(EditorMsg),
    NavigationMsg(NavigationOutput),
    CreateNote,
    DeleteNote,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Watermelon"),
            set_default_size: (1000, 700),
            set_size_request: (1200, 800),

            #[wrap(Some)]
            set_titlebar = &gtk::HeaderBar {
                pack_start = &gtk::Button {
                    set_icon_name: "list-add-symbolic",
                    set_tooltip_text: Some("New Note"),
                    connect_clicked => AppMsg::CreateNote,
                },
                pack_start = &gtk::Button {
                    set_icon_name: "user-trash-symbolic",
                    set_tooltip_text: Some("Delete Note"),
                    connect_clicked => AppMsg::DeleteNote,
                }
            },

            gtk::Paned {
                set_orientation: gtk::Orientation::Horizontal,
                set_position: 200,
                set_shrink_start_child: false,
                set_resize_start_child: false,

                #[wrap(Some)]
                set_start_child = model.navigation.widget(),

                #[wrap(Some)]
                set_end_child = &gtk::Paned {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_position: 250,
                    set_shrink_start_child: false,
                    set_resize_start_child: false,

                    #[wrap(Some)]
                    set_start_child = model.sidebar.widget(),

                    #[wrap(Some)]
                    set_end_child = model.editor.widget(),
                }
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Initialize DB
        let conn = crate::db::init_db("watermelon.db").expect("Failed to init DB");
        let repo = crate::db::note_repository::NoteRepository::new(conn);
        let note_service = Rc::new(NoteService::new(repo));

        // Load Notes
        let notes = note_service.get_all_notes().unwrap_or_default();

        // If empty, create a welcome note
        let notes = if notes.is_empty() {
            let welcome = note_service
                .create_note(
                    "Welcome to Watermelon".to_string(),
                    "This is your first note.".to_string(),
                )
                .expect("Failed to create welcome note");
            vec![welcome]
        } else {
            notes
        };

        let current_folder = "All Notes".to_string();

        let sidebar = Sidebar::builder()
            .launch(notes.clone())
            .forward(sender.input_sender(), AppMsg::SidebarMsg);

        let editor = Editor::builder()
            .launch(())
            .forward(sender.input_sender(), AppMsg::EditorMsg);

        let navigation = Navigation::builder()
            .launch(note_service.clone())
            .forward(sender.input_sender(), AppMsg::NavigationMsg);

        let model = App {
            navigation,
            sidebar,
            editor,
            notes,
            note_service,
            selected_index: None,
            current_folder,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::CreateNote => {
                let new_note = self
                    .note_service
                    .create_note("New Note".to_string(), "".to_string())
                    .expect("Failed to create note");

                self.notes.insert(0, new_note.clone()); // Add to top
                self.sidebar
                    .sender()
                    .send(SidebarMsg::UpdateNotes(self.notes.clone()))
                    .unwrap();

                // Select the new note
                self.selected_index = Some(0);
                self.sidebar
                    .sender()
                    .send(SidebarMsg::SelectNote(0))
                    .unwrap();
                self.editor
                    .sender()
                    .send(EditorMsg::LoadNote(new_note.title, new_note.content))
                    .unwrap();
            }
            AppMsg::DeleteNote => {
                if let Some(index) = self.selected_index {
                    if let Some(note) = self.notes.get(index) {
                        let _ = self.note_service.delete_note(note.id);
                        self.notes.remove(index);
                        self.sidebar
                            .sender()
                            .send(SidebarMsg::UpdateNotes(self.notes.clone()))
                            .unwrap();

                        // Select next note or clear
                        if self.notes.is_empty() {
                            self.selected_index = None;
                            self.editor
                                .sender()
                                .send(EditorMsg::LoadNote("".to_string(), "".to_string()))
                                .unwrap();
                        } else {
                            let new_index = if index >= self.notes.len() {
                                self.notes.len() - 1
                            } else {
                                index
                            };
                            self.selected_index = Some(new_index);
                            self.sidebar
                                .sender()
                                .send(SidebarMsg::SelectNote(new_index))
                                .unwrap();
                            // Note: Sidebar selection triggers LoadNote via AppMsg::SidebarMsg
                        }
                    }
                }
            }
            AppMsg::SidebarMsg(SidebarMsg::SelectNote(index)) => {
                // Save previous note if selected
                if let Some(prev_index) = self.selected_index {
                    if let Some(note) = self.notes.get(prev_index) {
                        let _ = self.note_service.update_note(note);
                    }
                }

                self.selected_index = Some(index);
                if let Some(note) = self.notes.get(index) {
                    self.editor
                        .sender()
                        .send(EditorMsg::LoadNote(
                            note.title.clone(),
                            note.content.clone(),
                        ))
                        .unwrap();
                }
            }
            AppMsg::SidebarMsg(SidebarMsg::UpdateNotes(_)) => {}
            AppMsg::SidebarMsg(SidebarMsg::Search(_)) => {}
            AppMsg::EditorMsg(EditorMsg::ToolbarMsg(_)) => {}
            AppMsg::EditorMsg(EditorMsg::UpdateContent(content)) => {
                if let Some(index) = self.selected_index {
                    if let Some(note) = self.notes.get_mut(index) {
                        note.content = content;
                        note.updated_at = chrono::Utc::now();
                        // DEFER SAVE: Don't save on every keystroke to prevent lag.
                        // We should save on note switch, app close, or a timer.
                        // For now, we rely on the in-memory update and save on switch.
                        // let _ = self.note_service.update_note(note);

                        // Update sidebar preview (optional, might be too frequent)
                        // self.sidebar.sender().send(SidebarMsg::UpdateNotes(self.notes.clone())).unwrap();
                    }
                }
            }
            AppMsg::EditorMsg(EditorMsg::UpdateTitle(title)) => {
                if let Some(index) = self.selected_index {
                    if let Some(note) = self.notes.get_mut(index) {
                        note.title = title;
                        note.updated_at = chrono::Utc::now();
                        let _ = self.note_service.update_note(note);

                        // Refresh sidebar to show new title
                        self.sidebar
                            .sender()
                            .send(SidebarMsg::UpdateNotes(self.notes.clone()))
                            .unwrap();
                    }
                }
            }
            AppMsg::EditorMsg(EditorMsg::LoadNote(_, _)) => {}
            AppMsg::EditorMsg(EditorMsg::InsertImage(_)) => {}
            AppMsg::EditorMsg(EditorMsg::Highlight) => {}
            AppMsg::EditorMsg(EditorMsg::InitTextView(_)) => {}
            AppMsg::NavigationMsg(output) => {
                match output {
                    NavigationOutput::FolderSelected(folder_name) => {
                        self.current_folder = folder_name.clone();
                        self.update_sidebar_notes();
                    }
                    NavigationOutput::MoveNote(note_id_str, folder_name) => {
                        if let Ok(note_id) = uuid::Uuid::parse_str(&note_id_str) {
                            // Find the note
                            if let Some(mut note) =
                                self.notes.iter().find(|n| n.id == note_id).cloned()
                            {
                                // Update folder
                                note.folder = Some(folder_name.clone());
                                note.updated_at = chrono::Utc::now();

                                // Save to DB
                                if let Err(e) = self.note_service.update_note(&note) {
                                } else {
                                    // Refresh notes
                                    if let Ok(notes) = self.note_service.get_all_notes() {
                                        self.notes = notes;
                                        self.update_sidebar_notes();
                                    }
                                }
                            }
                        }
                    }
                    NavigationOutput::RenameFolder(old_name, new_name) => {
                        if let Err(e) = self.note_service.rename_folder(&old_name, &new_name) {
                            eprintln!("Failed to rename folder: {}", e);
                        } else {
                            // Update current folder if it was the one renamed
                            if self.current_folder == old_name {
                                self.current_folder = new_name.clone();
                            }

                            // Refresh notes from DB to reflect folder name changes
                            if let Ok(notes) = self.note_service.get_all_notes() {
                                if let Some(first) = notes.first() {}
                                self.notes = notes;
                                self.update_sidebar_notes();
                            }
                        }
                    }
                    NavigationOutput::AddFolder(name) => {
                        if let Err(e) = self.note_service.add_folder(&name) {
                            eprintln!("Failed to add folder: {}", e);
                        } else {
                            // Reload folders in navigation?
                            // Navigation already added it to its list via push_back?
                            // Wait, Navigation::update for AddFolder emits this output but DOES NOT push to list.
                            // So we need to tell Navigation to reload or just push it.
                            // Actually, Navigation should probably optimistically add it, OR we reload.
                            // Let's reload the whole app? No.
                            // Let's just let Navigation handle the UI update, but we need to ensure it's saved.
                            // Re-reading Navigation::update:
                            // It emits AddFolder(new_name). It does NOT push to self.folders.
                            // So we need to save it here, and then somehow tell Navigation to update?
                            // Or, Navigation should have pushed it.
                            // Let's change Navigation to push it AND emit AddFolder.
                            // But wait, if I change Navigation to push it, then I don't need to do anything here except save it.
                        }
                    }
                }
            }
        }
    }
}

impl App {
    fn update_sidebar_notes(&mut self) {
        let filtered_notes: Vec<Note> = if self.current_folder == "All Notes" {
            self.notes.clone()
        } else if self.current_folder == "Untagged" {
            self.notes
                .iter()
                .filter(|n| n.folder.is_none())
                .cloned()
                .collect()
        } else {
            self.notes
                .iter()
                .filter(|n| n.folder.as_ref() == Some(&self.current_folder))
                .cloned()
                .collect()
        };

        self.sidebar
            .sender()
            .send(SidebarMsg::UpdateNotes(filtered_notes))
            .unwrap();
    }
}
