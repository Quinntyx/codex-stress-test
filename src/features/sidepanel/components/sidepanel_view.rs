use gtk::prelude::*;
use gtk::{Box, Orientation, Stack, StackSidebar};

// Material icons loaded via theme names for now

use crate::features::{file_tree::FileTree, placeholder::PlaceholderView};

pub struct SidePanel {
    pub container: Box,
    #[allow(dead_code)]
    stack: Stack,
    pub file_tree: FileTree,
}

impl SidePanel {
    pub fn new() -> Self {
        let stack = Stack::new();
        stack.set_hexpand(false);

        let file_tree = FileTree::new();
        stack.add_titled(&file_tree.container, Some("explorer"), "Explorer");
        stack.page(&file_tree.container).set_icon_name("folder");

        let scm = PlaceholderView::new("Not Yet Implemented");
        stack.add_titled(&scm.container, Some("scm"), "Source Control");
        stack.page(&scm.container).set_icon_name("repository");

        let extensions = PlaceholderView::new("Not Yet Implemented");
        stack.add_titled(&extensions.container, Some("extensions"), "Extensions");
        stack.page(&extensions.container).set_icon_name("list-add");

        let run = PlaceholderView::new("Not Yet Implemented");
        stack.add_titled(&run.container, Some("run"), "Run/Debug");
        stack.page(&run.container).set_icon_name("media-playback-start");

        let sidebar = StackSidebar::new();
        sidebar.set_stack(&stack);

        let container = Box::new(Orientation::Horizontal, 0);
        container.append(&sidebar);
        container.append(&stack);

        Self {
            container,
            stack,
            file_tree,
        }
    }
}
