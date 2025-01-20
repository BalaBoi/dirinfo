use anyhow::{Context, Result};

pub struct Info {
    n_files: usize,
    _size: u64,
}

impl Info {
    pub fn new(n_files: usize) -> Self {
        Self {
            n_files,
            _size: 0,
        }
    }
    pub fn display(&self, mut writer: impl std::io::Write) -> Result<()>{
        writeln!(writer, "Number of files: {}\n", self.n_files).context("Couldn't write info to writer")
    }
}