use anyhow::{Context, Result};

pub struct Info {
    n_files: usize,
    size: u64,
}

impl Info {
    pub fn new(size: u64, n_files: usize) -> Self {
        Self { n_files, size }
    }
    pub fn display(&self, mut writer: impl std::io::Write) -> Result<()> {
        writeln!(
            writer,
            "Number of files: {}\nLogical size of all the files: {} Bytes\n",
            self.n_files, self.size
        )
        .context("Couldn't write info to writer")
    }
}
