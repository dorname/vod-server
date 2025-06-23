use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub upload_dir: PathBuf,
    pub static_dir: PathBuf,
    pub host: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            upload_dir: PathBuf::from("uploads"),
            static_dir: PathBuf::from("static"),
            host: "0.0.0.0".to_string(),
            port: 8188,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_upload_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.upload_dir = dir.into();
        self
    }

    pub fn with_static_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.static_dir = dir.into();
        self
    }

    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
}
