use gtk4::prelude::*;
use vte4::prelude::*;
use gtk4::{gio, Application, Box, Orientation, PopoverMenu, GestureClick, ColorButton, gdk, DropDown, StringList, FontButton}; 
use libadwaita::{ApplicationWindow, HeaderBar, PreferencesWindow, PreferencesGroup, ActionRow};
use libadwaita::prelude::*;

use vte4::Terminal;
use std::rc::Rc;
use std::cell::RefCell;

use crate::config::{save_title_bar_setting, load_color_settings, save_color_settings, ColorSettings, ColorSchemePreset, load_app_settings, get_preset_colors, save_font_family_setting};

pub fn build_ui(app: &Application) {
    let terminal = Terminal::new();
    terminal.set_hexpand(true);
    terminal.set_vexpand(true);

    let app_settings = load_app_settings();
    let colors = app_settings.colors;
    let font_family = app_settings.font_family;
    let font_size = app_settings.font_size;
    let initial_title_bar_visible = app_settings.title_bar_visible;

    apply_color_settings(&terminal, &colors);
    let font_desc = pango::FontDescription::from_string(&format!("{} {}", font_family, font_size));
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
    let window_for_settings = window.clone();
    open_settings_action.connect_activate(move |_, _| {
        build_settings_window(&window_clone_for_settings, &terminal_clone_for_settings, &window_for_settings);
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
            if let Some(opacity) = colors.background_opacity {
                let rgba = gdk::RGBA::new(rgba.red(), rgba.green(), rgba.blue(), opacity as f32);
                terminal.set_color_background(&rgba);
            } else {
                terminal.set_color_background(&rgba);
            }
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

fn build_settings_window(parent: &ApplicationWindow, terminal: &Terminal, window: &ApplicationWindow) {
    let current_colors = Rc::new(RefCell::new(load_color_settings()));
    let app_settings = load_app_settings();
    let current_font_family = Rc::new(RefCell::new(app_settings.font_family));
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

    // background opacity control
    let opacity_adjustment = gtk4::Adjustment::new(
        current_colors.borrow().background_opacity.unwrap_or(1.0),
        0.0,
        1.0,
        0.01,
        0.1,
        0.0,
    );
    let opacity_scale = gtk4::Scale::new(gtk4::Orientation::Horizontal, Some(&opacity_adjustment));
    opacity_scale.set_digits(2);
    opacity_scale.set_hexpand(true);

    let opacity_row = ActionRow::builder()
        .title("Background Opacity")
        .activatable_widget(&opacity_scale)
        .build();
    opacity_row.add_suffix(&opacity_scale);
    general_group.add(&opacity_row);

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

    let font_group = PreferencesGroup::builder()
        .title("Font")
        .build();

    let font_button = FontButton::new();
    let current_font_desc_str = format!("{} {}", current_font_family.borrow(), current_font_size.borrow());
    let font_desc = pango::FontDescription::from_string(&current_font_desc_str);
    font_button.set_font_desc(&font_desc);

    let font_row = ActionRow::builder()
        .title("Font Family")
        .activatable_widget(&font_button)
        .build();
    font_row.add_suffix(&font_button);
    font_group.add(&font_row);

    let font_size_adjustment = gtk4::Adjustment::new(*current_font_size.borrow(), 6.0, 48.0, 1.0, 5.0, 0.0);
    let font_size_spin = gtk4::SpinButton::new(Some(&font_size_adjustment), 1.0, 0);
    font_size_spin.set_numeric(true);

    let font_size_row = ActionRow::builder()
        .title("Font Size")
        .activatable_widget(&font_size_spin)
        .build();
    font_size_row.add_suffix(&font_size_spin);
    font_group.add(&font_size_row);
    page.add(&font_group);

    let terminal_clone_for_font = terminal.clone();
    let current_font_family_clone = Rc::clone(&current_font_family);
    let current_font_size_clone_for_font_button = Rc::clone(&current_font_size);
    let font_size_spin_clone = font_size_spin.clone();
    font_button.connect_font_set(move |button| {
        if let Some(font_desc) = button.font_desc() {
            if let Some(family) = font_desc.family() {
                *current_font_family_clone.borrow_mut() = family.to_string();
            }
            if font_desc.size() > 0 {
                let new_size = font_desc.size() as f64 / pango::SCALE as f64;
                *current_font_size_clone_for_font_button.borrow_mut() = new_size;
                font_size_spin_clone.set_value(new_size);
            }
            let new_font_desc = format!("{} {}", current_font_family_clone.borrow(), current_font_size_clone_for_font_button.borrow());
            let font_desc = pango::FontDescription::from_string(&new_font_desc);
            terminal_clone_for_font.set_font(Some(&font_desc));
        }
    });

    let terminal_clone_for_font_size = terminal.clone();
    let current_font_size_clone = Rc::clone(&current_font_size);
    let current_font_family_clone_for_size = Rc::clone(&current_font_family);
    font_size_spin.connect_value_changed(move |spin| {
        let new_size = spin.value();
        *current_font_size_clone.borrow_mut() = new_size;
        let font_desc = pango::FontDescription::from_string(&format!("{} {}", current_font_family_clone_for_size.borrow(), new_size));
        terminal_clone_for_font_size.set_font(Some(&font_desc));
    });

    let general_group_clone = general_group.clone();
    let ansi_group_clone = ansi_group.clone();

    let terminal_clone_for_preset_apply = terminal.clone();
    let fg_button_clone_for_preset_update = fg_color_button.clone();
    let bg_button_clone_for_preset_update = bg_color_button.clone();
    let palette_buttons_clone_for_preset_update = palette_buttons.clone();
    let current_colors_clone_for_preset = Rc::clone(&current_colors);

    preset_dropdown.connect_selected_notify(move |dropdown| {
        let selected_idx = dropdown.selected();
        if let Some(preset) = ColorSchemePreset::all_presets().get(selected_idx as usize) {
            let preset_settings = get_preset_colors(preset);
            *current_colors_clone_for_preset.borrow_mut() = preset_settings.clone();

            if preset.name() == "Custom" {
                general_group_clone.set_visible(true);
                ansi_group_clone.set_visible(true);
            } else {
                general_group_clone.set_visible(false);
                ansi_group_clone.set_visible(false);
            }
            
            if let Some(fg_str) = &preset_settings.foreground {
                if let Ok(rgba) = fg_str.parse::<gdk::RGBA>() {
                    fg_button_clone_for_preset_update.set_rgba(&rgba);
                }
            }
            
            if let Some(bg_str) = &preset_settings.background {
                if let Ok(rgba) = bg_str.parse::<gdk::RGBA>() {
                    bg_button_clone_for_preset_update.set_rgba(&rgba);
                }
            }
            
            for (i, p_button) in palette_buttons_clone_for_preset_update.iter().enumerate() {
                if let Some(Some(color_str)) = preset_settings.palette.get(i) {
                    if let Ok(rgba) = color_str.parse::<gdk::RGBA>() {
                        p_button.set_rgba(&rgba);
                    }
                }
            }
            apply_color_settings(&terminal_clone_for_preset_apply, &preset_settings);
        }
    });

    if let Some(active_preset_name) = &current_colors.borrow().active_preset {
        if active_preset_name == "Custom" {
            general_group.set_visible(true);
            ansi_group.set_visible(true);
        } else {
            general_group.set_visible(false);
            ansi_group.set_visible(false);
        }
    } else {
        general_group.set_visible(true);
        ansi_group.set_visible(true);
    }

    let terminal_fg_clone = terminal.clone();
    let preset_dropdown_clone_fg = preset_dropdown.clone();
    let current_colors_clone_fg = Rc::clone(&current_colors);
    fg_color_button.connect_notify_local(Some("rgba"), move |button, _paramspec| {
        let rgba = button.rgba();
        let mut new_colors = load_color_settings(); 
        new_colors.foreground = Some(rgba.to_string());
        new_colors.active_preset = None; 
        preset_dropdown_clone_fg.set_selected(gtk4::INVALID_LIST_POSITION); 
        apply_color_settings(&terminal_fg_clone, &new_colors);
        
        let mut borrowed_current_colors = current_colors_clone_fg.borrow_mut();
        borrowed_current_colors.foreground = Some(rgba.to_string()); 
        borrowed_current_colors.active_preset = None;
    });

    let terminal_bg_clone = terminal.clone();
    let preset_dropdown_clone_bg = preset_dropdown.clone();
    let current_colors_clone_bg = Rc::clone(&current_colors);
    bg_color_button.connect_notify_local(Some("rgba"), move |button, _paramspec| {
        let rgba = button.rgba();
        let mut new_colors = load_color_settings(); 
        new_colors.background = Some(rgba.to_string());
        new_colors.active_preset = None; 
        preset_dropdown_clone_bg.set_selected(gtk4::INVALID_LIST_POSITION);
        apply_color_settings(&terminal_bg_clone, &new_colors);

        let mut borrowed_current_colors = current_colors_clone_bg.borrow_mut();
        borrowed_current_colors.background = Some(rgba.to_string()); 
        borrowed_current_colors.active_preset = None;
    });

    let terminal_opacity_clone = terminal.clone();
    let preset_dropdown_clone_opacity = preset_dropdown.clone();
    let current_colors_clone_opacity = Rc::clone(&current_colors);
    let window_clone = window.clone();
    opacity_scale.connect_value_changed(move |scale| {
        window_clone.set_opacity(scale.value());
        let opacity = scale.value();
        let mut new_colors = load_color_settings();
        new_colors.background_opacity = Some(opacity);
        new_colors.active_preset = None;
        preset_dropdown_clone_opacity.set_selected(gtk4::INVALID_LIST_POSITION);
        apply_color_settings(&terminal_opacity_clone, &new_colors);

        let mut borrowed_current_colors = current_colors_clone_opacity.borrow_mut();
        borrowed_current_colors.background_opacity = Some(opacity);
        borrowed_current_colors.active_preset = None;
    });

    for (i, p_button) in palette_buttons.iter().enumerate() {
        let p_button_clone = p_button.clone();
        let terminal_palette_clone = terminal.clone();
        let preset_dropdown_clone_palette = preset_dropdown.clone();
        let current_colors_clone_palette = Rc::clone(&current_colors);

        p_button_clone.connect_notify_local(Some("rgba"), move |btn, _|
            {
                let rgba = btn.rgba();
                let mut updated_colors = load_color_settings(); 
                if i < updated_colors.palette.len() {
                    updated_colors.palette[i] = Some(rgba.to_string());
                }
                updated_colors.active_preset = None; 
                preset_dropdown_clone_palette.set_selected(gtk4::INVALID_LIST_POSITION);
                apply_color_settings(&terminal_palette_clone, &updated_colors);
                
                let mut borrowed_current_colors = current_colors_clone_palette.borrow_mut();
                if i < borrowed_current_colors.palette.len() {
                    borrowed_current_colors.palette[i] = Some(rgba.to_string());
                }
                borrowed_current_colors.active_preset = None;
            }
        );
    }

    // Save font size to settings from setting
    let current_font_family_clone_for_save = Rc::clone(&current_font_family);
    let current_font_size_clone_for_save = Rc::clone(&current_font_size);
    preferences_window.connect_close_request(move |_window| {
        let mut settings_to_save = current_colors.borrow().clone();
        settings_to_save.active_preset = None; 
        save_color_settings(&settings_to_save);
        save_font_family_setting(&current_font_family_clone_for_save.borrow());
        crate::config::save_font_size_setting(*current_font_size_clone_for_save.borrow());
        glib::Propagation::Proceed
    });

    preferences_window.present();
}
