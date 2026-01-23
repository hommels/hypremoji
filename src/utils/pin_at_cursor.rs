use std::io::{BufRead, BufReader};

use crate::{cli::MousePosition, utils::get_config_dir};

pub fn set_pin_at_cursor(position: &MousePosition) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_dir()?.join("hypremoji.conf");

    let mut lines = vec![];
    
    if let Ok(file) = std::fs::File::open(&config_path) {
        let reader = BufReader::new(file);
        
        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            // Skip all move cursor rules (always remove them)
            if trimmed.starts_with("windowrulev2 = move cursor") 
                || (trimmed.starts_with("# windowrulev2 = move cursor")) {
                continue;
            }

            // Skip all move cursor rules (always remove them)
            if trimmed.starts_with("windowrule = move (cursor_x") 
                || trimmed.starts_with("# windowrule = move (cursor_x") {
                continue;
            }

            match position {
                MousePosition::None => {
                    // Uncomment static move rules
                    if trimmed.starts_with("# windowrule = move") 
                        && trimmed.contains("match:title ^(HyprEmoji)$") {
                        let uncommented = trimmed.trim_start_matches("# ").to_string();
                        lines.push(uncommented);
                    } else {
                        lines.push(line);
                    }
                }
                _ => {
                    // Comment static move rules
                    if trimmed.starts_with("windowrule = move") 
                        && trimmed.contains("match:title ^(HyprEmoji)$") {
                        lines.push(format!("# {}", trimmed));
                    } else {
                        lines.push(line);
                    }
                }
            }
        }

        // Add cursor rule if needed
        if *position != MousePosition::None {
            let cursor_rule = match position {
                MousePosition::Up => "windowrule = move (cursor_x-(window_w*0.5)) (cursor_y-(window_h*0.95)), match:title ^(HyprEmoji)$",
                MousePosition::Down => "windowrule = move (cursor_x-(window_w*0.5)) (cursor_y-(window_h*0.05)), match:title ^(HyprEmoji)$",
                _ => unreachable!(),
            };
            lines.push(cursor_rule.to_string());
            println!("Pin at cursor enabled: {:?} position", position);
        } else {
            println!("Pin at cursor disabled - using fixed position");
        }
    } else {
        return Err("Could not open hypremoji.conf".into());
    }

    // Write back to file
    std::fs::write(&config_path, lines.join("\n"))?;
    println!("Configuration updated successfully!");

    Ok(())
}