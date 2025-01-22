use anyhow::{Context, Result};

pub struct Info {
    n_files: usize,
    size: u64,
    blocks: u64,
}

impl Info {
    pub fn new(size: u64, n_files: usize, blocks: u64) -> Self {
        Self { n_files, size, blocks }
    }

    pub fn display(&self, mut writer: impl std::io::Write) -> Result<()> {
       writer.write(self.output().as_bytes())
        .context("Couldn't write info to writer")?;
        Ok(())
    }

    fn output(&self) -> String {
        format!(r#"Number of files: {}
Logical size of all the files: {}
Disk usage: {}
"#,
            self.n_files,
            file_size::fit_4(self.size),
            file_size::fit_4(self.blocks * 512))
    }
}
