use gtk::prelude::*;
use gtk::{Box, Button, Label, Orientation};

pub struct DashboardView {
    pub container: Box,
}

impl DashboardView {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 12);
        container.set_margin_top(24);
        container.set_margin_bottom(24);
        container.set_margin_start(24);
        container.set_margin_end(24);

        let title = Label::new(Some("Codeite"));
        title.add_css_class("title-1");
        container.append(&title);

        let new_project = Button::with_label("New Project");
        container.append(&new_project);

        let recent = Label::new(Some("Recent Projects"));
        recent.add_css_class("heading");
        container.append(&recent);

        let getting_started = Label::new(Some("Getting Started"));
        container.append(&getting_started);

        Self { container }
    }
}
