use gtk4::prelude::*;
use vte4::prelude::*;
use gtk4::{gio, Application, Box, Orientation, PopoverMenu, GestureClick, ColorButton, gdk, DropDown, StringList}; 
use libadwaita::{ApplicationWindow, HeaderBar, PreferencesWindow, PreferencesGroup, ActionRow};
use libadwaita::prelude::*;

use vte4::Terminal;
use std::rc::Rc;
use std::cell::RefCell;

use crate::config::{save_title_bar_setting, load_color_settings, save_color_settings, ColorSettings, ColorSchemePreset, load_app_settings};

pub fn build_ui(app: &Application) {
    let terminal = Terminal::new();
    terminal.set_hexpand(true);
    terminal.set_vexpand(true);

    let app_settings = load_app_settings();
    let colors = app_settings.colors;
    let font_size = app_settings.font_size;
    let initial_title_bar_visible = app_settings.title_bar_visible;

    apply_color_settings(&terminal, &colors);
    // Apply font size to terminal
    let font_desc = pango::FontDescription::from_string(&format!("Monospace {}", font_size));
    terminal.set_font(Some(&font_desc));

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
    menu.append(Some("Settings"), Some("win.open_settings")); 

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
    
    let open_settings_action = gio::SimpleAction::new("open_settings", None);
    let window_clone_for_settings = window.clone();
    let terminal_clone_for_settings = terminal.clone();
    open_settings_action.connect_activate(move |_, _| {
        build_settings_window(&window_clone_for_settings, &terminal_clone_for_settings);
    });
    window.add_action(&open_settings_action);

    let window_clone = window.clone();
    terminal.connect_child_exited(move |_terminal, _status| {
        window_clone.close();
    });

    window.present();
}

fn apply_color_settings(terminal: &Terminal, colors: &ColorSettings) {
    if let Some(fg_str) = &colors.foreground {
        if let Ok(rgba) = fg_str.parse::<gdk::RGBA>() {
            terminal.set_color_foreground(&rgba);
        } else {
            eprintln!("Failed to parse foreground color for apply: {}", fg_str);
        }
    }

    if let Some(bg_str) = &colors.background {
        if let Ok(rgba) = bg_str.parse::<gdk::RGBA>() {
            terminal.set_color_background(&rgba);
        } else {
            eprintln!("Failed to parse background color for apply: {}", bg_str);
        }
    }

    let mut palette_gdk: Vec<gdk::RGBA> = Vec::new();
    for (i, color_opt_str) in colors.palette.iter().enumerate() {
        if let Some(color_str) = color_opt_str {
            if let Ok(rgba) = color_str.parse::<gdk::RGBA>() {
                palette_gdk.push(rgba);
            } else {
                eprintln!("Failed to parse palette color {} ({}): {}", i, color_str, color_str);
            }
        }
    }

    if palette_gdk.len() == 16 {
        let fg_gdk = colors.foreground.as_ref().and_then(|s| s.parse::<gdk::RGBA>().ok());
        let bg_gdk = colors.background.as_ref().and_then(|s| s.parse::<gdk::RGBA>().ok());

        let palette_refs: Vec<&gdk::RGBA> = palette_gdk.iter().collect();
        terminal.set_colors(fg_gdk.as_ref(), bg_gdk.as_ref(), &palette_refs);
    } else if !palette_gdk.is_empty() {
        eprintln!(
            "Warning: Palette size is {}, not 16. Palette colors will not be applied.",
            palette_gdk.len()
        );
    }
}

fn build_settings_window(parent: &ApplicationWindow, terminal: &Terminal) {
    let current_colors = Rc::new(RefCell::new(load_color_settings()));
    let app_settings = load_app_settings();
    let current_font_size = Rc::new(RefCell::new(app_settings.font_size));

    let preferences_window = PreferencesWindow::builder()
        .title("Settings")
        .transient_for(parent)
        .modal(true)
        .build();

    let page = libadwaita::PreferencesPage::new();
    
    let preset_group = PreferencesGroup::builder()
        .title("Color Scheme Preset")
        .build();

    let preset_names: Vec<&str> = ColorSchemePreset::all_presets().iter().map(|p| p.name()).collect();
    let string_list = StringList::new(&preset_names);
    let preset_dropdown = DropDown::new(Some(string_list), gtk4::Expression::NONE);

    if let Some(active_preset_name) = &current_colors.borrow().active_preset {
        if let Some(pos) = preset_names.iter().position(|&name| name == active_preset_name) {
            preset_dropdown.set_selected(pos as u32);
        } else {
            preset_dropdown.set_selected(gtk4::INVALID_LIST_POSITION);
        }
    } else {
        preset_dropdown.set_selected(gtk4::INVALID_LIST_POSITION);
    }

    let preset_row = ActionRow::builder()
        .title("Select Preset")
        .activatable_widget(&preset_dropdown)
        .build();
    preset_row.add_suffix(&preset_dropdown);
    preset_group.add(&preset_row);
    page.add(&preset_group);
    
    let general_group = PreferencesGroup::builder()
        .title("General Colors (Overrides Preset)")
        .build();

    let fg_color_button = ColorButton::new();
    if let Some(fg_str) = &current_colors.borrow().foreground {
        if let Ok(rgba) = fg_str.parse::<gdk::RGBA>() {
            fg_color_button.set_rgba(&rgba);
        }
    }
    let fg_row = ActionRow::builder()
        .title("Foreground Color")
        .activatable_widget(&fg_color_button)
        .build();
    fg_row.add_suffix(&fg_color_button);
    general_group.add(&fg_row);

    let bg_color_button = ColorButton::new();
    if let Some(bg_str) = &current_colors.borrow().background {
        if let Ok(rgba) = bg_str.parse::<gdk::RGBA>() {
            bg_color_button.set_rgba(&rgba);
        }
    }
    let bg_row = ActionRow::builder()
        .title("Background Color")
        .activatable_widget(&bg_color_button)
        .build();
    bg_row.add_suffix(&bg_color_button);
    general_group.add(&bg_row);
    page.add(&general_group);
    
    let ansi_group = PreferencesGroup::builder()
        .title("ANSI Palette (Colors 0-15) (Overrides Preset)")
        .description("These colors are typically used by terminal applications.")
        .build();

    let mut palette_buttons: Vec<ColorButton> = Vec::new();
    for i in 0..16 {
        let color_button = ColorButton::new();
        if let Some(Some(color_str)) = current_colors.borrow().palette.get(i) {
            if let Ok(rgba) = color_str.parse::<gdk::RGBA>() {
                color_button.set_rgba(&rgba);
            }
        }
        let label_text = format!("Color {}", i);
        let row = ActionRow::builder()
            .title(&label_text)
            .activatable_widget(&color_button)
            .build();
        row.add_suffix(&color_button);
        ansi_group.add(&row);
        palette_buttons.push(color_button);
    }
    page.add(&ansi_group);

    preferences_window.add(&page);

    // Add font size control group
    let font_size_group = PreferencesGroup::builder()
        .title("Font Size")
        .build();

    let font_size_adjustment = gtk4::Adjustment::new(*current_font_size.borrow(), 6.0, 48.0, 1.0, 5.0, 0.0);
    let font_size_spin = gtk4::SpinButton::new(Some(&font_size_adjustment), 1.0, 0);
    font_size_spin.set_numeric(true);

    let font_size_row = ActionRow::builder()
        .title("Font Size")
        .activatable_widget(&font_size_spin)
        .build();
    font_size_row.add_suffix(&font_size_spin);
    font_size_group.add(&font_size_row);
    page.add(&font_size_group);

    let terminal_clone_for_font = terminal.clone();
    let current_font_size_clone = Rc::clone(&current_font_size);
    font_size_spin.connect_value_changed(move |spin| {
        let new_size = spin.value();
        *current_font_size_clone.borrow_mut() = new_size;
        let font_desc = pango::FontDescription::from_string(&format!("Monospace {}", new_size));
        terminal_clone_for_font.set_font(Some(&font_desc));
    });

    // Save font size to settings from setting
    let current_font_size_clone_for_save = Rc::clone(&current_font_size);
    preferences_window.connect_close_request(move |_window| {
        let mut settings_to_save = current_colors.borrow().clone();
        settings_to_save.active_preset = None; 
        save_color_settings(&settings_to_save);
        crate::config::save_font_size_setting(*current_font_size_clone_for_save.borrow());
        glib::Propagation::Proceed
    });

    preferences_window.present();
}
