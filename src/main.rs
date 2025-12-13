
mod app;
mod components;
mod core;
mod db;
mod models;
mod utils;

use app::App;

fn main() {
    println!("🍉 Watermelon is starting...");
    let app = relm4::RelmApp::new("com.aashiqumar.watermelon");

    // Load CSS
    relm4::set_global_css(include_str!("../assets/style.css"));

    // Add assets directory to icon theme search path (for development)
    if let Some(display) = gtk4::gdk::Display::default() {
        let theme = gtk4::IconTheme::for_display(&display);
        // Assuming we run from project root
        if let Ok(cwd) = std::env::current_dir() {
            let assets_path = cwd.join("assets");
            theme.add_search_path(assets_path);
        }
    }

    app.run::<App>(());
}
