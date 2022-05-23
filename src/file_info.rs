use std::hash::Hash;
use crate::line_col_lookup::LineColLookup;

#[derive(Debug, Default)]
pub struct FileInfo {
    pub content: String,
    pub path: String,
    pub package_root_path: String,
    line_col_lookup: LineColLookup,
}

impl FileInfo {
    pub fn new(content: String, path: String, package_root_path: String) -> Self {
        let line_col_lookup = LineColLookup::new(&content);

        Self { content, path, package_root_path, line_col_lookup }
    }

    pub fn get_line_col(&self, index: usize) -> Option<(usize, usize)> {
        self.line_col_lookup.get(index)
    }
}

impl PartialEq for FileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Hash for FileInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl Eq for FileInfo {
    
}