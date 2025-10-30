use gtk::{
    glib,
    prelude::{ButtonExt, WidgetExt},
    Button,
};

use crate::{
    ui::{create_generic_btn, IconName}, utils::save_current_window_state,
};

pub fn create_save_window_state_button() -> gtk::Button {
    let save_window_state_btn = create_generic_btn(IconName::AiFillPushpin, "pin-window-btn");
    save_window_state_btn.set_tooltip_text(Some("Save current location\nfor next time"));

    setup_save_locate_btn(save_window_state_btn.clone());
    save_window_state_btn
}

fn setup_save_locate_btn(btn: Button) {
    let btn_clone = btn.clone();
    btn.connect_clicked(move |_| {
        // Trigger click animation
        btn_clone.remove_css_class("shot-animation");

        // Force reflow to ensure GTK applies animation class again
        let btn_for_timeout = btn_clone.clone();
        glib::timeout_add_local_once(std::time::Duration::from_millis(10), move || {
            btn_for_timeout.add_css_class("shot-animation");

            // Remove animation class after it ends
            let btn_for_removal = btn_for_timeout.clone();
            glib::timeout_add_local_once(std::time::Duration::from_millis(1000), move || {
                btn_for_removal.remove_css_class("shot-animation");
            });
        });

        if let Err(e) = save_current_window_state() {
            println!("Error updating config: {}", e);
        } else {
            println!("Config updated successfully!");
        }
    });
}