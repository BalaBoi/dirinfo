mod dir_walker;

use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

use crate::info::Info;
use dir_walker::DirWalker;

pub struct Dir {
    path: PathBuf,
}

impl Dir {
    pub fn validate(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.is_dir() {
            return Err(anyhow!("Directory doesn't exist (dir: {})", path.display()));
        }
        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    pub fn info(&self) -> Result<Info> {
        let dir_walker = self.read_dir_recursive()?;
        let mut n_files = 0;
        let mut size = 0;
        let mut n_blocks = 0;
        for f_entry in dir_walker {
            let f_entry = f_entry?;
            n_files += 1;
            size += f_entry.size();
            n_blocks += f_entry.blocks();
        }
        Ok(Info::new(size, n_files, n_blocks))
    }

    fn read_dir_recursive(&self) -> Result<DirWalker> {
        DirWalker::new(&self.path)
    }
}
