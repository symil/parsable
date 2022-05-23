#[derive(Default)]
pub struct ParseOptions {
    pub file_path: Option<String>,
    pub package_root_path: Option<String>,
    pub comment_start: Option<&'static str>
}