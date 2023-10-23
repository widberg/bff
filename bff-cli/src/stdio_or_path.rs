use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum StdioOrPath {
    Stdio,
    Path(PathBuf),
}

impl From<&str> for StdioOrPath {
    fn from(value: &str) -> Self {
        match value {
            "-" => StdioOrPath::Stdio,
            value => StdioOrPath::Path(PathBuf::from(value)),
        }
    }
}
