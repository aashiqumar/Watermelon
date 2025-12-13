use relm4::prelude::*;

mod app;
mod components;
mod core;
mod db;
mod models;
mod utils;

use app::App;

fn main() {
    let app = relm4::RelmApp::new("com.aashiqumar.watermelon");

    // Load CSS
    relm4::set_global_css(include_str!("../assets/style.css"));

    app.run::<App>(());
}
