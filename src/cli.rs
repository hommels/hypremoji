use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "hypremoji")]
#[command(about = "A modern emoji picker for Hyprland, written in Rust + GTK4", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Reset configuration to defaults
    Reset,
    
    /// Initialize the mouse position for emoji picker
    InitInMouse {
        /// Position of the mouse for emoji picker
        #[arg(value_enum, default_value_t = MousePosition::Down)]
        position: MousePosition,
    },
}


#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum MousePosition {
    /// Window appears above the mouse cursor
    Up,
    /// Window appears below the mouse cursor
    Down,
    /// Remove windowrule for init in mouse
    None,
}