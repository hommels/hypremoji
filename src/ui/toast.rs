use gtk::{
    glib,
    prelude::*,
    Label, Box as GtkBox, Align,
};

pub struct Toast {
    pub(crate) container: GtkBox,
    pub(crate) label: Label,
}

impl Toast {
    pub fn new() -> Self {
        let container = GtkBox::new(gtk::Orientation::Horizontal, 0);
        container.set_halign(Align::Center);
        container.set_valign(Align::End);
        container.add_css_class("toast-container");

        let label = Label::new(Some("✅ lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec a diam lectus. Sed sit amet ipsum mauris."));
        label.add_css_class("toast-label");
        
        label.set_wrap(true);
        label.set_wrap_mode(gtk::pango::WrapMode::WordChar);
        label.set_max_width_chars(35);
        label.set_justify(gtk::Justification::Center); 
        label.set_halign(Align::Center);
        label.set_hexpand(true);

        container.append(&label);
        
        Self { container, label }
    }
    
    pub fn widget(&self) -> &GtkBox {
        &self.container
    }
    
    pub fn show_message(&self, message: &str) {
        self.label.set_text(message);
        self.container.remove_css_class("error");
        
        self.container.remove_css_class("show-toast");
        
        let container = self.container.clone();
        glib::timeout_add_local_once(std::time::Duration::from_millis(10), move || {
            container.add_css_class("show-toast");
        });
    }
    pub fn show_error(&self, message: &str) {
        self.label.set_text(message);
        self.container.add_css_class("error");
        
        self.container.remove_css_class("show-toast");
        
        let container = self.container.clone();
        glib::timeout_add_local_once(std::time::Duration::from_millis(10), move || {
            container.add_css_class("show-toast");
        });
    }
}