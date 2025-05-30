use gtk4::prelude::*;
use vte4::prelude::*;

use gtk4::{gio, Application, Box, Orientation, PopoverMenu, GestureClick};
use libadwaita::{ApplicationWindow, HeaderBar};
use vte4::Terminal;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

const APP_ID: &str = "com.betterecosystem.terminal";
const CONFIG_DIR: &str = ".config/better-terminal";
const CONFIG_FILE: &str = "better-terminal.conf";

fn get_config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|mut path| {
        path.push(CONFIG_DIR);
        path.push(CONFIG_FILE);
        path
    })
}

fn load_title_bar_setting() -> bool {
    if let Some(config_path) = get_config_path() {
        if config_path.exists() {
            if let Ok(mut file) = File::open(config_path) {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    for line in contents.lines() {
                        if line.starts_with("titlebar") {
                            let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
                            if parts.len() == 2 && parts[0] == "titlebar" {
                                return parts[1] == "true";
                            }
                        }
                    }
                }
            }
        }
    }
    true
}

fn save_title_bar_setting(is_visible: bool) {
    if let Some(config_path) = get_config_path() {
        if let Some(parent_dir) = config_path.parent() {
            if !parent_dir.exists() {
                if let Err(e) = fs::create_dir_all(parent_dir) {
                    eprintln!("Failed to create config directory: {}", e);
                    return;
                }
            }
        }
        
        let mut config_content = String::new();
        if config_path.exists() {
            if let Ok(mut file) = File::open(&config_path) {
                if file.read_to_string(&mut config_content).is_err() {
                    eprintln!("Failed to read existing config file, will overwrite.");
                    config_content.clear();
                }
            }
        }

        let mut new_lines = Vec::new();
        let mut titlebar_found = false;
        for line in config_content.lines() {
            if line.starts_with("titlebar") {
                new_lines.push(format!("titlebar = {}", is_visible));
                titlebar_found = true;
            } else {
                new_lines.push(line.to_string());
            }
        }

        if !titlebar_found {
            new_lines.push(format!("titlebar = {}", is_visible));
        }

        if let Ok(mut file) = File::create(config_path) {
            if let Err(e) = file.write_all(new_lines.join("\n").as_bytes()) {
                eprintln!("Failed to write to config file: {}", e);
            }
        } else {
            eprintln!("Failed to create or open config file for writing.");
        }
    }
}

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
    header_bar.set_show_end_title_buttons(true);

    let initial_title_bar_visible = load_title_bar_setting();
    header_bar.set_visible(initial_title_bar_visible);

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
        &glib::Variant::from(initial_title_bar_visible),
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
        save_title_bar_setting(new_state);
    });
    window.add_action(&toggle_title_bar_action);

    let window_clone = window.clone();
    terminal.connect_child_exited(move |_terminal, _status| {
        window_clone.close();
    });

    window.present();
}
