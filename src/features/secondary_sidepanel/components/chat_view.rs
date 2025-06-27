use gtk::prelude::*;
use gtk::{Box, Button, Entry, Orientation, TextView};

pub struct ChatView {
    pub container: Box,
}

impl ChatView {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 6);
        container.set_margin_top(6);
        container.set_margin_bottom(6);
        container.set_margin_start(6);
        container.set_margin_end(6);

        let text_view = TextView::new();
        text_view.set_editable(false);
        let entry = Entry::new();
        let send_button = Button::with_label("Send");
        let bottom = Box::new(Orientation::Horizontal, 6);
        bottom.append(&entry);
        bottom.append(&send_button);

        container.append(&text_view);
        container.append(&bottom);

        Self { container }
    }

    #[allow(dead_code)]
    pub fn send_message(&self, _message: &str) {
        // placeholder for LLM integration
    }
}
