use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Orientation};

mod features;
mod model;

use features::{
    bottom_panel::BottomPanel, dashboard::DashboardView, editor::EditorView,
    secondary_sidepanel::ChatView, sidepanel::SidePanel,
};
use model::{EditorModel, ProjectModel};
use std::cell::RefCell;
use std::rc::Rc;

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Codeite"));
    window.set_default_size(1200, 800);


    let root = Box::new(Orientation::Vertical, 0);

    let body = Box::new(Orientation::Horizontal, 0);

    // Models
    let project_model = Rc::new(RefCell::new(ProjectModel::default()));

    // Main side panel
    let main_panel = SidePanel::new();

    // Editor
    let editor_model = EditorModel::default();
    let editor_view = EditorView::new(editor_model);
    let dashboard = DashboardView::new();

    {
        let ft = main_panel.file_tree.clone();
        let pm = project_model.clone();
        let win = window.clone();
        dashboard.new_btn.connect_clicked(move |_| {
            #[allow(deprecated)]
            let dialog = gtk::FileChooserNative::new(
                Some("Select Folder"),
                Some(&win),
                gtk::FileChooserAction::SelectFolder,
                Some("Select"),
                Some("Cancel"),
            );
            #[allow(deprecated)]
            let response = glib::MainContext::default().block_on(dialog.run_future());
            if response == gtk::ResponseType::Accept {
                #[allow(deprecated)]
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        std::fs::create_dir_all(&path).ok();
                        pm.borrow_mut().root = Some(path.clone());
                        ft.load_path(&path);
                    }
                }
            }
            #[allow(deprecated)]
            dialog.destroy();
        });
    }

    {
        let ft = main_panel.file_tree.clone();
        let pm = project_model.clone();
        let win = window.clone();
        dashboard.open_btn.connect_clicked(move |_| {
            #[allow(deprecated)]
            let dialog = gtk::FileChooserNative::new(
                Some("Open Project"),
                Some(&win),
                gtk::FileChooserAction::SelectFolder,
                Some("Open"),
                Some("Cancel"),
            );
            #[allow(deprecated)]
            let response = glib::MainContext::default().block_on(dialog.run_future());
            if response == gtk::ResponseType::Accept {
                #[allow(deprecated)]
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        pm.borrow_mut().root = Some(path.clone());
                        ft.load_path(&path);
                    }
                }
            }
            #[allow(deprecated)]
            dialog.destroy();
        });
    }

    editor_view.add_page(&dashboard.container, "Dashboard");

    // Secondary panel with chat
    let chat_view = ChatView::new();

    // Secondary side panel container
    let secondary_revealer = gtk::Revealer::new();
    secondary_revealer.set_reveal_child(true);
    secondary_revealer.set_child(Some(&chat_view.container));

    body.append(&main_panel.container);
    body.append(&editor_view.container);
    body.append(&secondary_revealer);

    let bottom = BottomPanel::new();

    root.append(&body);
    root.append(&bottom.container);

    window.set_child(Some(&root));

    window.present();
}

fn main() {
    let app = Application::builder()
        .application_id("com.example.codeite")
        .build();

    app.connect_activate(build_ui);

    app.run();
}
