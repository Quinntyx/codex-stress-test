use gio::Cancellable;
use glib::SpawnFlags;
use gtk::prelude::*;
use gtk::{Box, Label, Notebook, Orientation};
use vte::prelude::*;
use vte::Terminal;

use crate::features::placeholder::PlaceholderView;

pub struct BottomPanel {
    pub container: Notebook,
}

impl BottomPanel {
    pub fn new() -> Self {
        let notebook = Notebook::new();
        notebook.set_hexpand(true);
        notebook.set_vexpand(false);

        notebook.append_page(
            &PlaceholderView::new("Problems").container,
            Some(&Label::new(Some("Problems"))),
        );
        notebook.append_page(
            &PlaceholderView::new("Output").container,
            Some(&Label::new(Some("Output"))),
        );
        notebook.append_page(
            &PlaceholderView::new("Debug Console").container,
            Some(&Label::new(Some("Debug Console"))),
        );

        // Terminal Tab
        let terminal = Terminal::new();
        if let Some(shell) = std::env::var_os("SHELL") {
            let shell_str = shell.to_string_lossy();
            terminal.spawn_async(
                vte::PtyFlags::DEFAULT,
                None::<&str>,
                &[&shell_str],
                &[],
                SpawnFlags::DEFAULT,
                || {},
                -1,
                Option::<&Cancellable>::None,
                |_| {},
            );
        }
        let term_box = Box::new(Orientation::Vertical, 0);
        term_box.append(&terminal);
        notebook.append_page(&term_box, Some(&gtk::Label::new(Some("Terminal"))));
        notebook.set_current_page(Some(3));

        Self {
            container: notebook,
        }
    }
}
