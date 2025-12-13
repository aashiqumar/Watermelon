use crate::core::note_service::NoteService;
use gtk::gdk;
use gtk::glib;
use gtk::prelude::*;
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct FolderRow {
    pub name: String,
    pub is_selected: bool,
    pub is_renaming: bool,
}

#[derive(Debug, Clone)]
pub enum FolderRowMsg {
    Select,
    DropNote(String), // Note ID
    StartRename,
    CommitRename(String),
    CancelRename,
    UpdateSelection(String), // Currently selected folder name
}

#[relm4::factory(pub)]
impl FactoryComponent for FolderRow {
    type Init = String;
    type Input = FolderRowMsg;
    type Output = FolderRowOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Stack {
            set_transition_type: gtk::StackTransitionType::Crossfade,

            add_child = &gtk::Button {
                set_css_classes: &["nav-item"],
                #[watch]
                set_css_classes: if self.is_selected { &["nav-item", "selected"] } else { &["nav-item"] },
                set_halign: gtk::Align::Fill,
                connect_clicked => FolderRowMsg::Select,

                add_controller = gtk::GestureClick {
                    set_button: gtk::gdk::BUTTON_PRIMARY,
                    set_propagation_phase: gtk::PropagationPhase::Capture,
                    connect_pressed[sender] => move |gesture, n_press, _, _| {
                        if n_press == 2 {
                            gesture.set_state(gtk::EventSequenceState::Claimed);
                            sender.input(FolderRowMsg::StartRename);
                        }
                    }
                },

                add_controller = gtk::DropTarget {
                    set_actions: gdk::DragAction::MOVE,
                    set_types: &[glib::Type::STRING],
                    connect_drop[sender] => move |_, value, _, _| {
                        if let Ok(note_id) = value.get::<String>() {
                            sender.input(FolderRowMsg::DropNote(note_id));
                            true
                        } else {
                            false
                        }
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,

                    gtk::Image {
                        set_icon_name: Some("folder-symbolic"),
                    },
                    gtk::Label {
                        #[watch]
                        set_label: &self.name,
                        set_hexpand: true,
                        set_halign: gtk::Align::Start,
                        set_css_classes: &["nav-item-label"],
                    },
                }
            } -> {
                set_name: "display",
            },

            add_child = &gtk::Entry {
                set_text: &self.name,
                set_halign: gtk::Align::Fill,
                set_hexpand: true,
                connect_map => move |entry| {
                    entry.grab_focus();
                },
                connect_activate[sender] => move |entry| {
                    sender.input(FolderRowMsg::CommitRename(entry.text().to_string()));
                },
                add_controller = gtk::EventControllerKey {
                    connect_key_pressed[sender] => move |_, key, _, _| {
                        if key == gtk::gdk::Key::Escape {
                            sender.input(FolderRowMsg::CancelRename);
                            return gtk::glib::Propagation::Stop;
                        }
                        gtk::glib::Propagation::Proceed
                    },
                },
                add_controller = gtk::EventControllerFocus {
                    connect_leave[sender] => move |controller| {
                        if let Some(widget) = controller.widget() {
                            if let Some(entry) = widget.downcast_ref::<gtk::Entry>() {
                                sender.input(FolderRowMsg::CommitRename(entry.text().to_string()));
                            }
                        }
                    }
                }
            } -> {
                set_name: "edit",
            },

            #[watch]
            set_visible_child_name: if self.is_renaming { "edit" } else { "display" },
        }
    }

    fn init_model(name: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            name,
            is_selected: false,
            is_renaming: false,
        }
    }

    fn update(&mut self, msg: Self::Input, sender: FactorySender<Self>) {
        match msg {
            FolderRowMsg::Select => {
                let _ = sender.output(FolderRowOutput::Selected(self.name.clone()));
            }
            FolderRowMsg::DropNote(note_id) => {
                let _ = sender.output(FolderRowOutput::NoteDropped(self.name.clone(), note_id));
            }
            FolderRowMsg::StartRename => {
                self.is_renaming = true;
            }
            FolderRowMsg::CommitRename(new_name) => {
                if !self.is_renaming {
                    return;
                }
                if !new_name.trim().is_empty() && new_name != self.name {
                    let old_name = self.name.clone();
                    self.name = new_name.clone();
                    self.is_renaming = false;

                    let _ = sender.output(FolderRowOutput::Renamed(old_name, new_name));
                } else {
                    self.is_renaming = false;
                }
            }
            FolderRowMsg::CancelRename => {
                self.is_renaming = false;
            }
            FolderRowMsg::UpdateSelection(selected_name) => {
                let was_selected = self.is_selected;
                self.is_selected = self.name == selected_name;

                // If we are not the selected folder, ensure we are not renaming
                if !self.is_selected {
                    self.is_renaming = false;
                }

                if was_selected != self.is_selected {}
            }
        }
    }
}

#[derive(Debug)]
pub enum FolderRowOutput {
    Selected(String),
    NoteDropped(String, String), // Folder Name, Note ID
    Renamed(String, String),     // Old Name, New Name
}

#[derive(Debug)]
pub enum NavigationMsg {
    SelectCategory(String),
    AddFolder,
    FolderOutput(FolderRowOutput),
}

pub struct Navigation {
    selected_category: String,
    folders: FactoryVecDeque<FolderRow>,
}

#[relm4::component(pub)]
impl SimpleComponent for Navigation {
    type Init = Rc<NoteService>;
    type Input = NavigationMsg;
    type Output = NavigationOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_css_classes: &["navigation-pane"],
            set_width_request: 200,

            // Header
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_all: 10,

                gtk::Label {
                    set_text: "FOLDERS",
                    set_css_classes: &["nav-header"],
                    set_halign: gtk::Align::Start,
                    set_hexpand: true,
                },

                gtk::Button {
                    set_icon_name: "list-add-symbolic", // Changed to standard add icon
                    set_tooltip_text: Some("New Folder"),
                    set_css_classes: &["flat", "nav-icon"],
                    connect_clicked => NavigationMsg::AddFolder,
                },
            },

            // Categories (Static)
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 2,
                set_margin_start: 10,
                set_margin_end: 10,

                gtk::Button {
                    #[watch]
                    set_css_classes: {

                        if model.selected_category == "All Notes" { &["nav-item", "selected"] } else { &["nav-item"] }
                    },
                    set_halign: gtk::Align::Fill,
                    connect_clicked => NavigationMsg::SelectCategory("All Notes".to_string()),

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 8,

                        gtk::Image {
                            set_icon_name: Some("text-x-generic-symbolic"),
                        },
                        gtk::Label {
                            set_label: "All Notes",
                            set_hexpand: true,
                            set_halign: gtk::Align::Start,
                            set_css_classes: &["nav-item-label"],
                        }
                    },
                },
            },

            // Folders (Dynamic)


            #[local_ref]
            folder_list -> gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 2,
                set_margin_start: 10,
                set_margin_end: 10,
            },
        }
    }

    fn init(
        note_service: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut folders = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), NavigationMsg::FolderOutput);

        // Load folders from DB
        if let Ok(db_folders) = note_service.get_folders() {
            for folder in db_folders {
                folders.guard().push_back(folder);
            }
        }

        let model = Navigation {
            selected_category: "All Notes".to_string(),
            folders,
        };

        let folder_list = model.folders.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            NavigationMsg::SelectCategory(cat) => {
                self.selected_category = cat.clone();
                self.folders
                    .broadcast(FolderRowMsg::UpdateSelection(cat.clone()));
                let _ = sender.output(NavigationOutput::FolderSelected(cat));
            }
            NavigationMsg::FolderOutput(output) => {
                match output {
                    FolderRowOutput::Selected(name) => {
                        self.selected_category = name.clone();
                        self.folders
                            .broadcast(FolderRowMsg::UpdateSelection(name.clone()));
                        let _ = sender.output(NavigationOutput::FolderSelected(name));
                    }
                    FolderRowOutput::NoteDropped(folder, note_id) => {
                        let _ = sender.output(NavigationOutput::MoveNote(note_id, folder));
                    }
                    FolderRowOutput::Renamed(old_name, new_name) => {
                        // Update selected category if it was the renamed folder
                        if self.selected_category == old_name {
                            self.selected_category = new_name.clone();
                        }
                        let _ = sender.output(NavigationOutput::RenameFolder(old_name, new_name));
                    }
                }
            }
            NavigationMsg::AddFolder => {
                // In a real app, this would show a dialog
                let new_name = format!("New Folder {}", self.folders.len() + 1);
                self.folders.guard().push_back(new_name.clone());
                let _ = sender.output(NavigationOutput::AddFolder(new_name));
            }
        }
    }
}

#[derive(Debug)]
pub enum NavigationOutput {
    FolderSelected(String),
    MoveNote(String, String),     // Note ID, Target Folder
    RenameFolder(String, String), // Old Name, New Name
    AddFolder(String),            // New Folder Name
}
