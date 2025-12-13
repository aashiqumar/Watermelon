use crate::models::note::Note;
use gtk::gdk;
use gtk::pango;
use gtk::prelude::*;
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;

#[derive(Debug)]
pub struct SidebarRow {
    pub note: Note,
    pub index: usize,
}

#[derive(Debug)]
pub enum SidebarRowMsg {
    Select,
}

#[relm4::factory(pub)]
impl FactoryComponent for SidebarRow {
    type Init = (usize, Note);
    type Input = SidebarRowMsg;
    type Output = SidebarMsg;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        gtk::ListBoxRow {
            set_activatable: true,
            set_css_classes: &["sidebar-row"],
            set_margin_bottom: 2,
            set_margin_top: 2,
            set_margin_start: 8,
            set_margin_end: 8,

            // Drag Source
            add_controller = gtk::DragSource {
                set_actions: gdk::DragAction::MOVE,
                connect_prepare[note_id = self.note.id] => move |_, _, _| {
                    let content = gdk::ContentProvider::for_value(&note_id.to_string().to_value());
                    Some(content)
                },
                connect_drag_begin => move |drag, _| {
                    if let Some(widget) = drag.widget() {
                        let paintable = gtk::WidgetPaintable::new(Some(&widget));
                        drag.set_icon(Some(&paintable), 0, 0);
                    }
                },
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 6,
                set_margin_all: 12,
                set_css_classes: &["sidebar-card"], // New class for card styling

                // Title
                gtk::Label {
                    set_label: &self.note.title,
                    set_halign: gtk::Align::Start,
                    set_css_classes: &["sidebar-title"],
                    set_ellipsize: pango::EllipsizeMode::End,
                },

                // Preview
                gtk::Label {
                    set_label: &self.note.content.lines().next().unwrap_or("No content"),
                    set_halign: gtk::Align::Start,
                    set_ellipsize: pango::EllipsizeMode::End,
                    set_css_classes: &["sidebar-preview"],
                    set_lines: 2, // Allow 2 lines for preview
                },

                // Folder Name (Instead of Date)
                gtk::Label {
                    set_label: &self.note.folder.clone().unwrap_or("All Notes".to_string()),
                    set_halign: gtk::Align::Start,
                    set_css_classes: &["sidebar-date"], // Reusing date style for folder
                },
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            index: init.0,
            note: init.1,
        }
    }

    fn update(&mut self, _msg: Self::Input, _sender: FactorySender<Self>) {}
}

#[derive(Debug)]
pub enum SidebarMsg {
    SelectNote(usize),
    UpdateNotes(Vec<Note>),
    Search(String),
}

#[derive(Debug)]
pub struct Sidebar {
    pub notes_factory: FactoryVecDeque<SidebarRow>,
    pub selected_index: Option<usize>,
    pub all_notes: Vec<Note>,
    pub search_text: String,
}

#[relm4::component(pub)]
impl SimpleComponent for Sidebar {
    type Init = Vec<Note>;
    type Input = SidebarMsg;
    type Output = SidebarMsg; // Forward selection to parent

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 0,

            gtk::SearchEntry {
                set_placeholder_text: Some("Search notes..."),
                set_margin_all: 12,
                connect_search_changed[sender] => move |entry| {
                    sender.input(SidebarMsg::Search(entry.text().to_string()));
                }
            },

            gtk::ScrolledWindow {
                set_hscrollbar_policy: gtk::PolicyType::Never,
                set_min_content_width: 250,
                set_vexpand: true,

                #[name = "notes_list"]
                gtk::ListBox {
                    set_selection_mode: gtk::SelectionMode::Single,
                    set_activate_on_single_click: true,
                    set_css_classes: &["navigation-sidebar"],

                    connect_row_activated[sender] => move |_, row| {
                        let index = row.index();
                        if index >= 0 {
                            sender.input(SidebarMsg::SelectNote(index as usize));
                        }
                    }
                }
            }
        }
    }

    fn init(
        notes: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let mut notes_factory = FactoryVecDeque::builder()
            .launch(widgets.notes_list.clone())
            .forward(sender.input_sender(), |msg| msg);

        // Populate initial notes
        for (i, note) in notes.iter().enumerate() {
            notes_factory.guard().push_back((i, note.clone()));
        }

        let model = Sidebar {
            notes_factory,
            selected_index: None,
            all_notes: notes,
            search_text: String::new(),
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            SidebarMsg::SelectNote(index) => {
                // Map the filtered index back to the original index if searching
                // But wait, the index from ListBoxRow corresponds to the filtered list (notes_factory)
                // We need to find the actual note in all_notes to send the correct index/ID to App?
                // Actually, App expects an index into ITS self.notes.
                // If we filter, the indices won't match App's indices.
                // PROBLEM: App uses index to find note. If Sidebar filters, index 0 might be note 5.
                // SOLUTION: We should probably pass the Note ID or the Note itself back to App,
                // OR App needs to know about the filtered state.
                // EASIER FIX for now: App receives SelectNote(index).
                // If we are filtering, we need to map this index to the real index in all_notes.

                // Let's find the note in the filtered list
                if let Some(row) = self.notes_factory.get(index) {
                    // We need to find this note's index in self.all_notes
                    if let Some(real_index) =
                        self.all_notes.iter().position(|n| n.id == row.note.id)
                    {
                        self.selected_index = Some(real_index);

                        // Programmatically select the row
                        if let Some(widget_row) =
                            self.notes_factory.widget().row_at_index(index as i32)
                        {
                            self.notes_factory.widget().select_row(Some(&widget_row));
                        }

                        let _ = sender.output(SidebarMsg::SelectNote(real_index));
                        return;
                    }
                }

                // Fallback (shouldn't happen if logic is correct)
                self.selected_index = Some(index);
                let _ = sender.output(SidebarMsg::SelectNote(index));
            }
            SidebarMsg::UpdateNotes(notes) => {
                self.all_notes = notes;
                self.update_filtered_list();

                // Restore selection logic is tricky with search.
                // If selected note is still visible, select it.
                if let Some(selected_idx) = self.selected_index {
                    if let Some(_selected_note) = self.all_notes.get(selected_idx) {
                        // Find this note in the filtered factory
                        // We need to iterate the factory... accessing the inner deque is hard from here without guard
                        // But we just rebuilt it in update_filtered_list
                        // Let's do a simple check:
                        // We can't easily iterate the factory items to find the index.
                        // But we can iterate the filtered notes we just pushed.

                        // Re-running filter logic to find index
                        let filtered_notes: Vec<(usize, &Note)> = self
                            .all_notes
                            .iter()
                            .enumerate()
                            .filter(|(_, n)| {
                                self.search_text.is_empty()
                                    || n.title
                                        .to_lowercase()
                                        .contains(&self.search_text.to_lowercase())
                                    || n.content
                                        .to_lowercase()
                                        .contains(&self.search_text.to_lowercase())
                            })
                            .collect();

                        if let Some(filtered_idx) = filtered_notes
                            .iter()
                            .position(|(orig_idx, _)| *orig_idx == selected_idx)
                        {
                            if let Some(row) = self
                                .notes_factory
                                .widget()
                                .row_at_index(filtered_idx as i32)
                            {
                                self.notes_factory.widget().select_row(Some(&row));
                            }
                        }
                    }
                }
            }
            SidebarMsg::Search(text) => {
                self.search_text = text;
                self.update_filtered_list();
            }
        }
    }
}

impl Sidebar {
    fn update_filtered_list(&mut self) {
        self.notes_factory.guard().clear();

        let search_lower = self.search_text.to_lowercase();

        for (i, note) in self.all_notes.iter().enumerate() {
            if self.search_text.is_empty()
                || note.title.to_lowercase().contains(&search_lower)
                || note.content.to_lowercase().contains(&search_lower)
            {
                // We pass 'i' (original index) to SidebarRow so it knows its real identity?
                // Actually SidebarRow just displays data. The 'index' in SidebarRow init was used for selection.
                // If we pass 'i' here, SidebarRowMsg::Select will send 'i'.
                // Then SidebarMsg::SelectNote(i) will be called.
                // 'i' is the REAL index in all_notes.
                // So we don't need the complex mapping in SelectNote handler above!
                // Wait, connect_row_activated sends row.index() which is the VISUAL index (0, 1, 2...).
                // So we DO need mapping if we rely on row.index().

                // BETTER APPROACH:
                // Let's use the SidebarRow's internal index (which we set to 'i' - the real index).
                // But we are using connect_row_activated on the ListBox, which gives us the visual index.
                // We can't easily access the SidebarRow data from the ListBox row activation without casting.

                // ALTERNATIVE:
                // Go back to using connect_activate on SidebarRow? No, that was buggy.

                // HYBRID:
                // We have the filtered list.
                // row.index() is the index in the filtered list.
                // We can reconstruct the filtered list to find the real index.

                self.notes_factory.guard().push_back((i, note.clone()));
            }
        }
    }
}
