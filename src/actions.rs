use crate::{
    cli::Commands,
    utils
};

pub fn handle_command(command: &Commands) {
    match command {
        Commands::Reset => {
            println!("Resetting Hypremoji configuration...");

            if let Err(e) = utils::reset_config() {
                eprintln!("Error resetting configuration: {}", e);
                std::process::exit(1);
            } else {
                println!("Configuration reset successfully!");
                return;
            }
        }
        Commands::InitInMouse { position } => {
            println!("Setting mouse position to: {:?}", position);

            if let Err(e) = utils::set_pin_at_cursor(position) {
                eprintln!("Error setting mouse position: {}", e);
                std::process::exit(1);
            } else {
                println!("Mouse position set successfully!");
                return;
            }
        }
    }
}