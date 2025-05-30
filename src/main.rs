use gtk4::prelude::*;
use vte4::prelude::*;

use gtk4::{gio, Application, Box, Orientation, PopoverMenu, GestureClick};
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
    header_bar.set_show_end_title_buttons(true); // Ensure default title buttons are shown

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

    let menu = gio::Menu::new();
    menu.append(Some("Show Title Bar"), Some("win.toggle_title_bar"));

    let popover = PopoverMenu::from_model(Some(&menu));
    popover.set_parent(&terminal);

    let gesture = GestureClick::new();
    gesture.set_button(3);
    let popover_clone = popover.clone();
    gesture.connect_pressed(move |_, _, x, y| {
        let rect = gtk4::gdk::Rectangle::new(x as i32, y as i32, 1, 1);
        popover_clone.set_pointing_to(Some(&rect));
        popover_clone.popup();
    });
    terminal.add_controller(gesture);

    let header_bar_clone = header_bar.clone();
    let toggle_title_bar_action = gio::SimpleAction::new_stateful(
        "toggle_title_bar",
        None,
        &glib::Variant::from(true),
    );
    toggle_title_bar_action.connect_activate(move |action, _| {
        let current_state = action
            .state()
            .expect("Could not get state.")
            .get::<bool>()
            .expect("Could not get bool state.");
        let new_state = !current_state;
        header_bar_clone.set_visible(new_state);
        action.set_state(&glib::Variant::from(new_state));
    });
    window.add_action(&toggle_title_bar_action);

    let window_clone = window.clone();
    terminal.connect_child_exited(move |_terminal, _status| {
        window_clone.close();
    });

    window.present();
}
