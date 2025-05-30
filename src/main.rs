mod config;
mod ui;

use gtk4::prelude::*;
use gtk4::Application;

const APP_ID: &str = "com.betterecosystem.terminal";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| {
        let _ = libadwaita::init();
    });

    app.connect_activate(ui::build_ui);
    app.run();
}
