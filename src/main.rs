use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, HeaderBar, Orientation};

mod features;
mod model;

use features::{
    bottom_panel::BottomPanel, dashboard::DashboardView, editor::EditorView,
    secondary_sidepanel::ChatView, sidepanel::SidePanel,
};
use model::EditorModel;

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Codeite"));
    window.set_default_size(1200, 800);

    let header = HeaderBar::new();
    let title_label = gtk::Label::new(Some("Codeite"));
    header.set_title_widget(Some(&title_label));
    window.set_titlebar(Some(&header));

    let root = Box::new(Orientation::Vertical, 0);

    let body = Box::new(Orientation::Horizontal, 0);

    // Main side panel
    let main_panel = SidePanel::new();

    // Editor
    let editor_model = EditorModel::default();
    let editor_view = EditorView::new(editor_model);
    let dashboard = DashboardView::new();
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
