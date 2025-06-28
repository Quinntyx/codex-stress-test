use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Clone)]
pub struct FileTab {
    pub path: PathBuf,
    pub content: String,
    pub unsaved: bool,
}

#[allow(dead_code)]
impl FileTab {
    pub fn new(path: PathBuf, content: String) -> Self {
        Self {
            path,
            content,
            unsaved: false,
        }
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct EditorModel {
    tabs: Vec<FileTab>,
    pub current: Option<usize>,
}

#[derive(Default, Clone)]
pub struct ProjectModel {
    pub root: Option<PathBuf>,
}

#[allow(dead_code)]
impl EditorModel {
    pub fn add_tab(&mut self, tab: FileTab) {
        self.tabs.push(tab);
        self.current = Some(self.tabs.len() - 1);
    }

    pub fn current_tab_mut(&mut self) -> Option<&mut FileTab> {
        self.current.and_then(|i| self.tabs.get_mut(i))
    }
}
