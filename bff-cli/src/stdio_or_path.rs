use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum StdioOrPath {
    Stdio,
    Path(PathBuf),
}

impl From<&str> for StdioOrPath {
    fn from(value: &str) -> Self {
        match value {
            "-" => Self::Stdio,
            value => Self::Path(PathBuf::from(value)),
        }
    }
}
