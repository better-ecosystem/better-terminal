use gtk4::prelude::*;
use vte4::prelude::*;

use gtk4::{gio, Application, Box, Orientation};
use libadwaita::{ApplicationWindow, HeaderBar};
use vte4::Terminal;

const APP_ID: &str = "com.betterecosystem.terminal";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| {
        let _ = libadwaita::init();
    });

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let terminal = Terminal::new();
    terminal.set_hexpand(true);
    terminal.set_vexpand(true);
    let default_shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    terminal.spawn_async(
        vte4::PtyFlags::DEFAULT,
        None,
        &[&default_shell],
        &[],
        glib::SpawnFlags::DEFAULT,
        || {},
        -1,
        None::<&gio::Cancellable>,
        |result| {
            if let Err(e) = result {
                eprintln!("Failed to spawn shell: {}", e);
            }
        },
    );

    let header_bar = HeaderBar::new();

    let content_box = Box::new(Orientation::Vertical, 0);
    content_box.append(&header_bar);
    content_box.append(&terminal);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Better Terminal")
        .default_width(600)
        .default_height(400)
        .content(&content_box)
        .build();

    window.present();
}
