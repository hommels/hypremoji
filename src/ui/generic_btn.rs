use gtk::prelude::{ButtonExt, WidgetExt};
use gtk::{Button, IconTheme};

use crate::utils::get_assets_base_path;

pub enum IconName {
    AiFillPushpin,
    MouseUp,
    MouseDown,
    Resize,
}

impl IconName {
    // pub fn as_str(&self) -> &'static str {
    //     match self {
    //         IconName::AiFillPushpin => "AiFillPushpin-symbolic.svg",
    //         IconName::MouseUp => "MouseUp-symbolic.svg",
    //         IconName::MouseDown => "MouseDown-symbolic.svg",
    //     }
    // }
    pub fn as_str_without_extension(&self) -> &'static str {
        match self {
            IconName::AiFillPushpin => "AiFillPushpin-symbolic",
            IconName::MouseUp => "MouseUp-symbolic",
            IconName::MouseDown => "MouseDown-symbolic",
            IconName::Resize => "Resize-symbolic",
        }
    }
}

pub fn create_generic_btn(icon_name: IconName, css_class: &str) -> gtk::Button {
    let btn = gtk::Button::new();

    btn.set_icon_name(icon_name.as_str_without_extension());
    
    btn.add_css_class(css_class);
    btn.add_css_class("generic-btn");
    
    btn
}

pub fn register_custom_icons() -> Result<(), Box<dyn std::error::Error>> {
    let icon_theme = IconTheme::for_display(&gtk::gdk::Display::default().unwrap());
    let assets_path = get_assets_base_path()?;
    let icons_path = assets_path.join("icons");
    
    icon_theme.add_search_path(icons_path.to_str().unwrap());
    
    Ok(())
}

pub fn update_btn_icon(btn: &Button, icon: IconName) {
    btn.set_icon_name(icon.as_str_without_extension());
}
