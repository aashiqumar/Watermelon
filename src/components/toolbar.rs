use gtk::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
pub enum ToolbarMsg {
    Checkbox,
    BulletList,
    NumberedList,
    Bold,
    Italic,
    Highlight,
    Link,
}

#[derive(Debug)]
pub struct Toolbar {
    list_popover: gtk::Popover,
}

#[relm4::component(pub)]
impl SimpleComponent for Toolbar {
    type Init = ();
    type Input = ToolbarMsg;
    type Output = ToolbarMsg;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 2,
            set_css_classes: &["toolbar-pill"],

            // Checkbox
            gtk::Button {
                set_icon_name: "checkbox-checked-symbolic", // Better task icon
                set_tooltip_text: Some("Task List"),
                set_css_classes: &["flat", "toolbar-btn"],
                connect_clicked => ToolbarMsg::Checkbox,
            },

            // List Dropdown
            gtk::MenuButton {
                set_icon_name: "view-list-bullet-symbolic",
                set_tooltip_text: Some("Lists"),
                set_css_classes: &["flat", "toolbar-btn"],
                set_popover: Some(&model.list_popover),
            },

            gtk::Separator {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_start: 4,
                set_margin_end: 4,
            },

            // Bold
            gtk::Button {
                set_icon_name: "format-text-bold-symbolic",
                set_tooltip_text: Some("Bold (Ctrl+B)"),
                set_css_classes: &["flat", "toolbar-btn"],
                connect_clicked => ToolbarMsg::Bold,
            },

            // Italic
            gtk::Button {
                set_icon_name: "format-text-italic-symbolic",
                set_tooltip_text: Some("Italic (Ctrl+I)"),
                set_css_classes: &["flat", "toolbar-btn"],
                connect_clicked => ToolbarMsg::Italic,
            },

            // Highlight
            gtk::Button {
                set_icon_name: "user-bookmarks-symbolic", // Looks like a marker/tag
                set_tooltip_text: Some("Highlight"),
                set_css_classes: &["flat", "toolbar-btn"],
                connect_clicked => ToolbarMsg::Highlight,
            },

            gtk::Separator {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_start: 4,
                set_margin_end: 4,
            },

            // Link
            gtk::Button {
                set_icon_name: "insert-link-symbolic",
                set_tooltip_text: Some("Link"),
                set_css_classes: &["flat", "toolbar-btn"],
                connect_clicked => ToolbarMsg::Link,
            },
        }
    }

    fn init(
        _: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Create Popovers manually
        let list_popover = gtk::Popover::builder().build();
        let l_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(2)
            .build();

        let bullet_btn = gtk::Button::builder()
            .label("Bullet List")
            .css_classes(["flat"])
            .build();
        let sender_clone = sender.clone();
        bullet_btn.connect_clicked(move |_| {
            sender_clone.input(ToolbarMsg::BulletList);
        });

        let num_btn = gtk::Button::builder()
            .label("Numbered List")
            .css_classes(["flat"])
            .build();
        let sender_clone = sender.clone();
        num_btn.connect_clicked(move |_| {
            sender_clone.input(ToolbarMsg::NumberedList);
        });

        l_box.append(&bullet_btn);
        l_box.append(&num_btn);
        list_popover.set_child(Some(&l_box));

        let model = Toolbar { list_popover };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        // Close popovers on selection
        match msg {
            ToolbarMsg::BulletList | ToolbarMsg::NumberedList => self.list_popover.popdown(),
            _ => {}
        }
        let _ = sender.output(msg);
    }
}
