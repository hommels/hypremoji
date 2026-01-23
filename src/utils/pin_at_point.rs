use std::io::{BufRead, BufReader};

use crate::utils::{get_config_dir, get_current_offset, get_hypremoji_client};

pub fn save_current_window_state() -> Result<(), Box<dyn std::error::Error>> {
    let hypremoji_client = get_hypremoji_client();
    let screens_size = get_current_offset();

    let (mut at_x, mut at_y) = hypremoji_client.at;
    let (size_x, size_y) = hypremoji_client.size;

    // Subtract screen offset
    at_x -= screens_size.0;
    at_y -= screens_size.1;

    println!(
        "Saving current location: ({}, {}) with size: ({}, {})",
        at_x, at_y, size_x, size_y
    );

    let new_position_rule = format!("windowrule = move {} {}, match:title ^(HyprEmoji)$", at_x, at_y);
    let new_size_rule = format!("windowrule = size {} {}, match:title ^(HyprEmoji)$", size_x, size_y);

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

            // Skip all move cursor rules (always remove them)
            if trimmed.starts_with("windowrule = move (cursor_x") 
                || trimmed.starts_with("# windowrule = move (cursor_x") {
                continue;
            }

            // Replace move rule (commented or not)
            if (trimmed.starts_with("windowrule = move") || trimmed.starts_with("# windowrule = move"))
                && trimmed.contains("match:title ^(HyprEmoji)$") {
                lines.push(position_rule.to_string());
                found_position = true;
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