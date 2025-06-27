use gtk::prelude::*;
use gtk::{Box, Label, Orientation};

pub struct PlaceholderView {
    pub container: Box,
}

impl PlaceholderView {
    pub fn new(text: &str) -> Self {
        let container = Box::new(Orientation::Vertical, 0);
        container.set_hexpand(true);
        container.set_vexpand(true);
        let label = Label::new(Some(text));
        container.append(&label);
        Self { container }
    }
}
