use gtk::prelude::*;
use gtk::{Box, Expander, Label, Orientation, ScrolledWindow};
use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct FileTree {
    pub container: ScrolledWindow,
    root: Box,
}

impl FileTree {
    pub fn new() -> Self {
        let root_box = Box::new(Orientation::Vertical, 0);
        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(&root_box));
        scrolled.set_hexpand(false);
        scrolled.set_vexpand(true);
        Self {
            container: scrolled,
            root: root_box,
        }
    }

    pub fn load_path<P: AsRef<Path>>(&self, path: P) {
        while let Some(child) = self.root.first_child() {
            self.root.remove(&child);
        }
        self.add_dir(path.as_ref(), &self.root);
    }

    #[allow(clippy::only_used_in_recursion)]
    fn add_dir(&self, path: &Path, parent: &Box) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                let name = entry
                    .file_name()
                    .to_string_lossy()
                    .to_string();
                if p.is_dir() {
                    let expander = Expander::new(Some(&name));
                    let child_box = Box::new(Orientation::Vertical, 0);
                    expander.set_child(Some(&child_box));
                    parent.append(&expander);
                    self.add_dir(&p, &child_box);
                } else {
                    let label = Label::new(Some(&name));
                    label.set_halign(gtk::Align::Start);
                    label.set_margin_start(12);
                    parent.append(&label);
                }
            }
        }
    }
}
