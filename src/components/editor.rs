use crate::components::toolbar::{Toolbar, ToolbarMsg};
use gtk::glib;
use gtk::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
pub enum EditorMsg {
    UpdateContent(String),
    UpdateTitle(String),
    LoadNote(String, String), // Title, Content
    ToolbarMsg(ToolbarMsg),
    InsertImage(String),
    Highlight,
    InitTextView(gtk::TextView),
}

#[derive(Debug)]
pub struct Editor {
    pub content: String,
    pub title: String,
    pub should_reload_buffer: bool,
    pub should_update_title: bool,
    pub buffer: gtk::TextBuffer,
    pub toolbar: Controller<Toolbar>,
    pub text_view: Option<gtk::TextView>,
}

impl Editor {
    fn create_buffer(content: &str) -> gtk::TextBuffer {
        let buffer = gtk::TextBuffer::builder().text(content).build();

        // Define tags
        buffer.create_tag(Some("bold"), &[("weight", &700)]); // 700 = Bold
        buffer.create_tag(Some("italic"), &[("style", &gtk::pango::Style::Italic)]);
        buffer.create_tag(Some("strikethrough"), &[("strikethrough", &true)]);
        buffer.create_tag(
            Some("code"),
            &[
                ("family", &"monospace"),
                ("paragraph-background", &"#2C3E50"), // Dark Slate (Block level)
                ("foreground", &"#ECF0F1"),           // Light Grey text
                ("weight", &600),
                ("left-margin", &10),
                ("right-margin", &10),
                ("pixels-above-lines", &5),
                ("pixels-below-lines", &5),
            ],
        );

        // Syntax Highlighting Tags (Dracula Theme inspired)
        buffer.create_tag(
            Some("keyword"),
            &[("foreground", &"#FF79C6"), ("weight", &700)],
        ); // Pink
        buffer.create_tag(
            Some("type"),
            &[("foreground", &"#8BE9FD"), ("weight", &700)],
        ); // Cyan
        buffer.create_tag(Some("string"), &[("foreground", &"#F1FA8C")]); // Yellow
        buffer.create_tag(
            Some("comment"),
            &[
                ("foreground", &"#6272A4"),
                ("style", &gtk::pango::Style::Italic),
            ],
        ); // Grey

        buffer.create_tag(
            Some("hidden"),
            &[("invisible", &true), ("editable", &false)],
        );

        // Highlight (Marker)
        buffer.create_tag(
            Some("highlight"),
            &[
                ("background", &"#FFFACD"), // LemonChiffon
                ("foreground", &"#000000"),
            ],
        );

        buffer
    }

    fn highlight_buffer(buffer: &gtk::TextBuffer) {
        // Simple regex-based highlighter for demonstration
        // In a real app, this should be more robust and maybe incremental

        let (start, end) = buffer.bounds();
        let _text = buffer.text(&start, &end, true);

        // We only want to highlight inside code blocks
        // Iterate over "code" tag ranges
        if let Some(code_tag) = buffer.tag_table().lookup("code") {
            let mut iter = buffer.start_iter();

            while iter.forward_to_tag_toggle(Some(&code_tag)) {
                if !iter.toggles_tag(Some(&code_tag)) {
                    // This is an end toggle, continue
                    continue;
                }

                // We found a start of a code block
                let mut end_iter = iter;
                if !end_iter.forward_to_tag_toggle(Some(&code_tag)) {
                    end_iter = buffer.end_iter();
                }

                // Extract text in this block
                let block_text = buffer.text(&iter, &end_iter, false);
                let block_start_offset = iter.offset();

                // Keywords
                let keywords = [
                    "fn", "struct", "pub", "impl", "let", "mut", "use", "mod", "match", "if",
                    "else", "return", "val", "var", "def", "class", "trait", "enum", "type",
                    "const", "static", "async", "await",
                ];
                for keyword in keywords {
                    let pattern = format!(r"\b{}\b", keyword);
                    if let Ok(re) = regex::Regex::new(&pattern) {
                        for mat in re.find_iter(&block_text) {
                            let start_byte = mat.start();
                            let end_byte = mat.end();

                            // Convert byte offsets to char offsets
                            let start_char_offset = block_text[..start_byte].chars().count() as i32;
                            let end_char_offset = block_text[..end_byte].chars().count() as i32;

                            let mut k_start =
                                buffer.iter_at_offset(block_start_offset + start_char_offset);
                            let mut k_end =
                                buffer.iter_at_offset(block_start_offset + end_char_offset);
                            buffer.apply_tag_by_name("keyword", &k_start, &k_end);
                        }
                    }
                }

                // Types (Capitalized words)
                if let Ok(re) = regex::Regex::new(r"\b[A-Z][a-zA-Z0-9_]*\b") {
                    for mat in re.find_iter(&block_text) {
                        let start_byte = mat.start();
                        let end_byte = mat.end();

                        let start_char_offset = block_text[..start_byte].chars().count() as i32;
                        let end_char_offset = block_text[..end_byte].chars().count() as i32;

                        let mut t_start =
                            buffer.iter_at_offset(block_start_offset + start_char_offset);
                        let mut t_end = buffer.iter_at_offset(block_start_offset + end_char_offset);
                        buffer.apply_tag_by_name("type", &t_start, &t_end);
                    }
                }

                // Strings
                if let Ok(re) = regex::Regex::new(r#""[^"]*""#) {
                    for mat in re.find_iter(&block_text) {
                        let start_byte = mat.start();
                        let end_byte = mat.end();

                        let start_char_offset = block_text[..start_byte].chars().count() as i32;
                        let end_char_offset = block_text[..end_byte].chars().count() as i32;

                        let mut s_start =
                            buffer.iter_at_offset(block_start_offset + start_char_offset);
                        let mut s_end = buffer.iter_at_offset(block_start_offset + end_char_offset);
                        buffer.apply_tag_by_name("string", &s_start, &s_end);
                    }
                }

                // Comments
                if let Ok(re) = regex::Regex::new(r"//.*") {
                    for mat in re.find_iter(&block_text) {
                        let start_byte = mat.start();
                        let end_byte = mat.end();

                        let start_char_offset = block_text[..start_byte].chars().count() as i32;
                        let end_char_offset = block_text[..end_byte].chars().count() as i32;

                        let mut c_start =
                            buffer.iter_at_offset(block_start_offset + start_char_offset);
                        let mut c_end = buffer.iter_at_offset(block_start_offset + end_char_offset);
                        buffer.apply_tag_by_name("comment", &c_start, &c_end);
                    }
                }

                iter = end_iter;
            }
        }
    }
}

#[relm4::component(pub)]
impl SimpleComponent for Editor {
    type Init = ();
    type Input = EditorMsg;
    type Output = EditorMsg; // Forward changes to parent for autosave

    view! {
        gtk::Overlay {
            set_hexpand: true,
            set_vexpand: true,
            set_css_classes: &["editor-container"],

            // Shortcuts
            add_controller = gtk::EventControllerKey {
                connect_key_pressed[sender] => move |_controller, keyval, _keycode, state| {
                    let is_ctrl = state.contains(gtk::gdk::ModifierType::CONTROL_MASK);
                    if is_ctrl {
                        match keyval {
                            gtk::gdk::Key::b | gtk::gdk::Key::B => {
                                sender.input(EditorMsg::ToolbarMsg(ToolbarMsg::Bold));
                                return gtk::glib::Propagation::Stop;
                            }
                            gtk::gdk::Key::i | gtk::gdk::Key::I => {
                                sender.input(EditorMsg::ToolbarMsg(ToolbarMsg::Italic));
                                return gtk::glib::Propagation::Stop;
                            }
                            _ => {}
                        }
                    }
                    gtk::glib::Propagation::Proceed
                }
            },

            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                // Title Input
                gtk::Entry {
                    set_placeholder_text: Some("Note Title"),
                    set_css_classes: &["title-1", "editor-title"],
                    set_margin_top: 12,
                    set_margin_start: 32,
                    set_margin_end: 32,
                    set_margin_bottom: 12,

                    #[track(model.should_update_title)]
                    set_text: &model.title,

                    connect_changed[sender] => move |entry| {
                        sender.input(EditorMsg::UpdateTitle(entry.text().to_string()));
                    }
                },

                // Content Area
                gtk::ScrolledWindow {
                    set_vexpand: true,
                    set_hexpand: true,

                    #[name = "text_view"]
                    gtk::TextView {
                        set_wrap_mode: gtk::WrapMode::Word,
                        set_top_margin: 24,
                        set_bottom_margin: 80, // Extra margin for floating toolbar
                        set_left_margin: 32,
                        set_right_margin: 32,
                        set_css_classes: &["editor-content"],

                        #[track(model.should_reload_buffer)]
                        set_buffer: Some(&model.buffer),
                    }
                }
            },

            add_overlay = model.toolbar.widget() {
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::End,
                set_margin_bottom: 24,
            },
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let buffer = Self::create_buffer("");

        let toolbar = Toolbar::builder()
            .launch(())
            .forward(sender.input_sender(), EditorMsg::ToolbarMsg);

        let model = Editor {
            content: String::new(),
            title: String::new(),
            should_reload_buffer: false,
            should_update_title: false,
            buffer,
            toolbar,
            text_view: None,
        };

        let widgets = view_output!();

        // Connect to initial buffer
        widgets
            .text_view
            .buffer()
            .connect_changed(glib::clone!(@strong sender => move |buffer| {
                let (start, end) = buffer.bounds();
                let text = buffer.text(&start, &end, true).to_string();
                sender.input(EditorMsg::UpdateContent(text));
            }));

        // Connect on buffer replacement
        widgets.text_view.connect_buffer_notify(glib::clone!(@strong sender => move |text_view| {
            text_view.buffer().connect_changed(glib::clone!(@strong sender => move |_| {
                sender.input(EditorMsg::Highlight);
            }));

            text_view.buffer().connect_changed(glib::clone!(@strong sender, @strong text_view => move |_| {
                let buffer = text_view.buffer();
                let (start, end) = buffer.bounds();
                let text = buffer.text(&start, &end, true);
                sender.input(EditorMsg::UpdateContent(text.to_string()));
            }));
        }));

        // Send the text_view widget to the model for later use (e.g., image insertion)

        sender.input(EditorMsg::InitTextView(widgets.text_view.clone()));

        // Add Click Handler for Checkboxes manually
        let gesture = gtk::GestureClick::new();
        gesture.connect_pressed(glib::clone!(@strong sender => move |gesture, _n_press, x, y| {
            let text_view = gesture.widget().unwrap().downcast::<gtk::TextView>().unwrap();
            let buffer = text_view.buffer();

            let (x_buffer, y_buffer) = text_view.window_to_buffer_coords(gtk::TextWindowType::Widget, x as i32, y as i32);
            if let Some(iter) = text_view.iter_at_location(x_buffer, y_buffer) {
                let mut start = iter;
                start.backward_chars(3);
                let mut end = iter;
                end.forward_chars(3);

                let text = buffer.text(&start, &end, false);

                if let Some(pos) = text.find("[ ]") {
                    let mut match_start = start;
                    match_start.forward_chars(pos as i32);
                    let mut match_end = match_start;
                    match_end.forward_chars(3);
                    buffer.delete(&mut match_start, &mut match_end);
                    buffer.insert(&mut match_start, "[x]");
                } else if let Some(pos) = text.find("[x]") {
                    let mut match_start = start;
                    match_start.forward_chars(pos as i32);
                    let mut match_end = match_start;
                    match_end.forward_chars(3);
                    buffer.delete(&mut match_start, &mut match_end);
                    buffer.insert(&mut match_start, "[ ]");
                }
            }
        }));
        widgets.text_view.add_controller(gesture);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        self.should_reload_buffer = false;
        self.should_update_title = false;
        match msg {
            EditorMsg::UpdateContent(text) => {
                // Prevent infinite loop if update comes from buffer
                if self.content != text {
                    self.content = text.clone();
                    // Do NOT update tracker here. We don't want to reset the buffer while typing.
                    let _ = sender.output(EditorMsg::UpdateContent(text));
                }
            }
            EditorMsg::UpdateTitle(text) => {
                if self.title != text {
                    self.title = text.clone();
                    let _ = sender.output(EditorMsg::UpdateTitle(text));
                }
            }
            EditorMsg::Highlight => {
                // Self::highlight_buffer(&self.buffer);
            }
            EditorMsg::InitTextView(view) => {
                self.text_view = Some(view);
            }
            EditorMsg::LoadNote(title, content) => {
                self.title = title;
                self.content = content.clone();

                // Create new buffer with tags
                self.buffer = Self::create_buffer(&content);
                // Self::highlight_buffer(&self.buffer); // Syntax highlighting disabled for now
                self.should_reload_buffer = true; // Only update buffer on load
                self.should_update_title = true;
            }
            EditorMsg::InsertImage(path) => {
                let buffer = &self.buffer;
                let (mut start, mut end) = buffer.selection_bounds().unwrap_or_else(|| {
                    let offset = buffer.cursor_position();
                    let iter = buffer.iter_at_offset(offset);
                    (iter, iter)
                });

                let start_mark = buffer.create_mark(None, &start, true);
                buffer.delete(&mut start, &mut end);

                let mut iter = buffer.iter_at_mark(&start_mark);
                let text = format!("![Image]({})", path);
                buffer.insert(&mut iter, &text);

                // Insert Image Preview
                if let Some(text_view) = &self.text_view {
                    let mut anchor_iter = buffer.iter_at_mark(&start_mark);
                    // Move after the inserted text to show preview below/after
                    anchor_iter.forward_chars(text.chars().count() as i32);

                    buffer.insert(&mut anchor_iter, "\n"); // Newline for image
                    let anchor = buffer.create_child_anchor(&mut anchor_iter);

                    let picture = gtk::Picture::for_filename(&path);
                    picture.set_content_fit(gtk::ContentFit::ScaleDown);
                    picture.set_height_request(200); // Limit height
                    picture.set_margin_top(10);
                    picture.set_margin_bottom(10);

                    text_view.add_child_at_anchor(&picture, &anchor);
                }

                let start_iter = buffer.iter_at_mark(&start_mark);
                let mut cursor = start_iter;
                cursor.forward_chars(text.chars().count() as i32 + 1); // +1 for newline
                buffer.place_cursor(&cursor);
                buffer.delete_mark(&start_mark);
            }
            EditorMsg::ToolbarMsg(msg) => {
                let buffer = &self.buffer;
                let (mut start, mut end) = buffer.selection_bounds().unwrap_or_else(|| {
                    let offset = buffer.cursor_position();
                    let iter = buffer.iter_at_offset(offset);
                    (iter, iter)
                });

                let has_selection = start.offset() != end.offset();

                match msg {
                    ToolbarMsg::Checkbox => {
                        buffer.insert(&mut start, "- [ ] ");
                    }

                    ToolbarMsg::Highlight => {
                        if has_selection {
                            let mut end_iter = end;
                            buffer.insert(&mut end_iter, "==");
                            let end_mark = buffer.create_mark(None, &end_iter, true);

                            let mut start_iter = start;
                            buffer.insert(&mut start_iter, "==");
                            let start_mark = buffer.create_mark(None, &start_iter, true);

                            let start_iter = buffer.iter_at_mark(&start_mark);
                            let end_iter = buffer.iter_at_mark(&end_mark);

                            let mut inner_start = start_iter;
                            inner_start.forward_chars(2);
                            let mut inner_end = end_iter;
                            inner_end.backward_chars(2);

                            buffer.apply_tag_by_name("highlight", &inner_start, &inner_end);
                            buffer.apply_tag_by_name("hidden", &start_iter, &inner_start);
                            buffer.apply_tag_by_name("hidden", &inner_end, &end_iter);

                            buffer.delete_mark(&start_mark);
                            buffer.delete_mark(&end_mark);
                        } else {
                            buffer.insert(&mut start, "====");
                            let mut iter = start;
                            iter.backward_chars(2);
                            buffer.place_cursor(&iter);
                        }
                    }
                    ToolbarMsg::Bold => {
                        if has_selection {
                            let mut end_iter = end;
                            buffer.insert(&mut end_iter, "**");
                            let end_mark = buffer.create_mark(None, &end_iter, true);

                            let mut start_iter = start;
                            buffer.insert(&mut start_iter, "**");
                            let start_mark = buffer.create_mark(None, &start_iter, true);

                            let start_iter = buffer.iter_at_mark(&start_mark);
                            let end_iter = buffer.iter_at_mark(&end_mark);

                            let mut inner_start = start_iter;
                            inner_start.forward_chars(2);
                            let mut inner_end = end_iter;
                            inner_end.backward_chars(2);

                            buffer.apply_tag_by_name("bold", &inner_start, &inner_end);
                            buffer.apply_tag_by_name("hidden", &start_iter, &inner_start);
                            buffer.apply_tag_by_name("hidden", &inner_end, &end_iter);

                            buffer.delete_mark(&start_mark);
                            buffer.delete_mark(&end_mark);
                        } else {
                            buffer.insert(&mut start, "****");
                            let mut iter = start;
                            iter.backward_chars(2);
                            buffer.place_cursor(&iter);
                        }
                    }
                    ToolbarMsg::Italic => {
                        if has_selection {
                            let mut end_iter = end;
                            buffer.insert(&mut end_iter, "_");
                            let end_mark = buffer.create_mark(None, &end_iter, true);

                            let mut start_iter = start;
                            buffer.insert(&mut start_iter, "_");
                            let start_mark = buffer.create_mark(None, &start_iter, true);

                            let start_iter = buffer.iter_at_mark(&start_mark);
                            let end_iter = buffer.iter_at_mark(&end_mark);

                            let mut inner_start = start_iter;
                            inner_start.forward_chars(1);
                            let mut inner_end = end_iter;
                            inner_end.backward_chars(1);

                            buffer.apply_tag_by_name("italic", &inner_start, &inner_end);
                            buffer.apply_tag_by_name("hidden", &start_iter, &inner_start);
                            buffer.apply_tag_by_name("hidden", &inner_end, &end_iter);

                            buffer.delete_mark(&start_mark);
                            buffer.delete_mark(&end_mark);
                        } else {
                            buffer.insert(&mut start, "__");
                            let mut iter = start;
                            iter.backward_chars(1);
                            buffer.place_cursor(&iter);
                        }
                    }
                    ToolbarMsg::BulletList => {
                        buffer.insert(&mut start, "- ");
                    }
                    ToolbarMsg::NumberedList => {
                        buffer.insert(&mut start, "1. ");
                    }
                    ToolbarMsg::Link => {
                        if has_selection {
                            let text = buffer.text(&start, &end, false);
                            let new_text = format!("[{}]()", text);

                            let start_mark = buffer.create_mark(None, &start, true);
                            buffer.delete(&mut start, &mut end);

                            let mut iter = buffer.iter_at_mark(&start_mark);
                            buffer.insert(&mut iter, &new_text);

                            let start_iter = buffer.iter_at_mark(&start_mark);
                            let mut cursor = start_iter;
                            cursor.forward_chars(new_text.chars().count() as i32 - 1);
                            buffer.place_cursor(&cursor);
                            buffer.delete_mark(&start_mark);
                        } else {
                            let start_mark = buffer.create_mark(None, &start, true);
                            let mut iter = buffer.iter_at_mark(&start_mark);
                            buffer.insert(&mut iter, "[]()");

                            let start_iter = buffer.iter_at_mark(&start_mark);
                            let mut cursor = start_iter;
                            cursor.forward_chars(1); // Inside brackets
                            buffer.place_cursor(&cursor);
                            buffer.delete_mark(&start_mark);
                        }
                    }
                }
            }
        }
    }
}
