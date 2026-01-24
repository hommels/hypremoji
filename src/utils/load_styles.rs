use std::{fs, path::PathBuf};

use gtk::{gdk::Display, CssProvider};

use crate::utils::{get_config_dir, reset_css};

pub fn load_css(custom_css_path: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let provider = CssProvider::new();

    let css_path = if let Some(custom_path) = custom_css_path {
        let path = PathBuf::from(custom_path);
        if !path.exists() {
            return Err(format!("CSS file not found: {}", path.display()).into());
        }
        path
    } else {
        let config_dir_path = get_config_dir()?;
        let user_main_css_path = config_dir_path.join("style.css");

        if !user_main_css_path.exists() {
            reset_css(&config_dir_path)?;
            println!("Created default CSS at: {}", user_main_css_path.display());
        }

        user_main_css_path
    };

    if css_path.exists() {
        // En lugar de leerlo a string, pasamos la ruta directamente
        provider.load_from_path(css_path.to_str().unwrap());
    } else {
        // Solo usas string para el fallback por defecto
        provider.load_from_string("window { background-color: #282A36; }");
    }

    gtk::style_context_add_provider_for_display(
        &Display::default().ok_or("No display found")?,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    Ok(())
}
