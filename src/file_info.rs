use std::hash::Hash;
use crate::LineColLookup;

#[derive(Debug, Default)]
pub struct FileInfo {
    pub content: String,
    pub path: String,
    pub package_root_path: String,
}

impl FileInfo {
    pub fn compute_lookup_index(&self) -> LineColLookup {
        LineColLookup::new(&self.content)
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