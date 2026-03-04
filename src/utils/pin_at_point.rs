use std::io::{BufRead, BufReader};

use crate::{config::paste_config::PinMode, utils::{get_config_dir, get_current_offset, get_hypremoji_client}};

pub fn change_pin_type(pin_mode: PinMode) -> Result<(), Box<dyn std::error::Error>> {
    let hypremoji_client = get_hypremoji_client();
    let screens_size = get_current_offset();

    let (mut at_x, mut at_y) = hypremoji_client.at;
    let (size_x, size_y) = hypremoji_client.size;

    // Subtract screen offset
    at_x -= screens_size.0;
    at_y -= screens_size.1;

    // Generate position rule based on pin mode
    let new_position_rule = match pin_mode {
        PinMode::Point => {
            println!("Setting Point mode at: ({}, {})", at_x, at_y);
            format!("windowrule = move {} {}, match:title ^(HyprEmoji)$", at_x, at_y)
        },
        PinMode::CursorUp => {
            println!("Setting CursorUp mode");
            "windowrule = move (cursor_x-(window_w*0.5)) (cursor_y-(window_h*0.05)), match:title ^(HyprEmoji)$".to_string()
        },
        PinMode::CursorDown => {
            println!("Setting CursorDown mode");
            "windowrule = move (cursor_x-(window_w*0.5)) (cursor_y-(window_h*0.95)), match:title ^(HyprEmoji)$".to_string()
        },
    };

    let new_size_rule = format!("windowrule = size {} {}, match:title ^(HyprEmoji)$", size_x, size_y);

    println!("Updating to {} mode with size: ({}, {})", 
        match pin_mode {
            PinMode::Point => "Point",
            PinMode::CursorUp => "CursorUp",
            PinMode::CursorDown => "CursorDown",
        },
        size_x, size_y
    );

    update_position_and_size(&new_position_rule, &new_size_rule)
}

fn update_position_and_size(
    position_rule: &str,
    size_rule: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_dir()?.join("hypremoji.conf");

    let mut lines = vec![];
    let mut found_position = false;
    let mut found_size = false;

    if let Ok(file) = std::fs::File::open(&config_path) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            // Replace size rule
            if trimmed.starts_with("windowrule = size") 
                && trimmed.contains("match:title ^(HyprEmoji)$") {
                lines.push(size_rule.to_string());
                found_size = true;
                continue;
            }

            // Replace any move rule (Point, CursorUp, or CursorDown)
            // This catches all variations: numeric positions, cursor_x formulas, etc.
            if (trimmed.starts_with("windowrule = move") || trimmed.starts_with("# windowrule = move"))
                && trimmed.contains("match:title ^(HyprEmoji)$") {
                // Only add the position rule once
                if !found_position {
                    lines.push(position_rule.to_string());
                    found_position = true;
                }
                // Skip this line (we're replacing it)
                continue;
            }

            // Keep all other lines
            lines.push(line);
        }
    } else {
        return Err("Could not open hypremoji.conf".into());
    }

    // Add missing rules at the end
    if !found_size {
        lines.push(size_rule.to_string());
        println!("Added missing size rule");
    }

    if !found_position {
        lines.push(position_rule.to_string());
        println!("Added missing position rule");
    }

    // Write back to file
    std::fs::write(&config_path, lines.join("\n"))?;
    println!("Position and size updated successfully!");

    Ok(())
}