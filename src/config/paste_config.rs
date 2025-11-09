use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use crate::utils::{path_utils::get_config_dir, reset_config::reset_paste_config};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasteConfig {
    pub shift_paste_apps: Vec<String>,
}

impl Default for PasteConfig {
    fn default() -> Self {
        PasteConfig {
            shift_paste_apps: vec![
                "kitty".to_string(),
                "alacritty".to_string(),
                "foot".to_string(),
                "wezterm".to_string(),
                "terminator".to_string(),
                "tilix".to_string(),
                "gnome-terminal".to_string(),
                "konsole".to_string(),
                "xterm".to_string(),
                "urxvt".to_string(),
                "st".to_string(),
                "tmux".to_string(),
                "rio".to_string(),
            ],
        }
    }
}

impl PasteConfig {
    pub fn needs_shift_for_paste(&self, window_class: &str) -> bool {
        let class_lower = window_class.to_lowercase();
        self.shift_paste_apps
            .iter()
            .any(|app| class_lower.contains(&app.to_lowercase()))
    }
}

fn get_paste_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("paste_config.json"))
}

pub fn load_paste_config() -> PasteConfig {
    match get_paste_config_path() {
        Ok(config_path) => {
            // If it doesn't exist, try to copy from default file
            if !config_path.exists() {
                println!("paste_config.json not found. Creating from default file...");
                if let Ok(config_dir) = get_config_dir() {
                    if let Err(e) = reset_paste_config(&config_dir) {
                        eprintln!("Failed to copy default paste_config.json: {}. Using hardcoded values.", e);
                        return PasteConfig::default();
                    }
                } else {
                    eprintln!("Could not get config directory. Using default values.");
                    return PasteConfig::default();
                }
            }

            // Read and parse the file
            match fs::read_to_string(&config_path) {
                Ok(content) => {
                    match serde_json::from_str::<PasteConfig>(&content) {
                        Ok(config) => {
                            println!("Paste configuration loaded from: {:?}", config_path);
                            return config;
                        }
                        Err(e) => {
                            eprintln!(
                                "Failed to parse paste_config.json: {}. Using default configuration.",
                                e
                            );
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Failed to read paste_config.json: {}. Using default configuration.",
                        e
                    );
                }
            }
        }
        Err(e) => {
            eprintln!(
                "Failed to get config path: {}. Using default configuration.",
                e
            );
        }
    }

    PasteConfig::default()
}