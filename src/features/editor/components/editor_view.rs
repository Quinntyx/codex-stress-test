use gtk::prelude::*;
use gtk::{Label, Notebook, TextView};

use crate::model::{EditorModel, FileTab};

pub struct EditorView {
    pub container: Notebook,
    #[allow(dead_code)]
    model: EditorModel,
}

impl EditorView {
    pub fn new(model: EditorModel) -> Self {
        let notebook = Notebook::new();
        notebook.set_hexpand(true);
        notebook.set_vexpand(true);
        Self {
            container: notebook,
            model,
        }
    }

    #[allow(dead_code)]
    pub fn add_tab(&mut self, tab: FileTab) {
        let text_view = TextView::new();
        text_view.buffer().set_text(&tab.content);
        let label_text = tab.path.file_name().unwrap().to_string_lossy();
        let label = Label::new(Some(&label_text));
        let page_num = self.container.append_page(&text_view, Some(&label));
        self.container.set_current_page(Some(page_num));
        self.model.add_tab(tab);
    }

    pub fn add_page<W: IsA<gtk::Widget> + 'static>(&self, widget: &W, title: &str) {
        let label = Label::new(Some(title));
        self.container.append_page(widget, Some(&label));
    }

    #[allow(dead_code)]
    pub fn open_file(&mut self, path: &std::path::Path) {
        if let Ok(content) = std::fs::read_to_string(path) {
            let tab = FileTab::new(path.to_path_buf(), content);
            self.add_tab(tab);
        }
    }
}
