use clap::Parser;
use std::env;
use gtk::prelude::*;
use gtk::Application;

mod category;
mod services;
mod ui;
mod utils;
mod cli;
mod actions;
mod config;

use crate::cli::Cli;
use crate::config::load_config;
use crate::ui::build_ui;
use crate::ui::generic_btn::register_custom_icons;
use crate::utils::load_css;

fn main() {
    let cli = Cli::parse();

    if let Some(command) = &cli.command {
        actions::handle_command(command);
        return
    }
    let app_config = load_config();

    let cb_manager = utils::get_clipboard_manager(&app_config);

    let app = Application::builder()
        .application_id("dev.musagy.hypremoji")
        .build();

    let style_path = cli.style.clone();
    app.connect_startup(move |_| {
        if let Err(e) = load_css(style_path.as_deref()) {
            eprintln!("Error loading CSS: {}", e);
        }
    });

    let cb_manager_clone = cb_manager.clone();
    app.connect_activate(move |app| {
        register_custom_icons().expect("Failed to register icons");
        build_ui(app, cb_manager_clone.clone(), &app_config);
    });
    let gtk_args: Vec<String> = env::args().take(1).collect();
    app.run_with_args(&gtk_args);

    cb_manager.send_emoji_to_focused_window();
}
