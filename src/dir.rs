use anyhow::{anyhow, Result};
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use crate::info::Info;

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
        let (size, n_files) = n_files(&self.path)?;
        Ok(Info::new(size, n_files))
    }
}

fn n_files(path: &Path) -> Result<(u64, usize)> {
    let mut n = 0;
    let mut size = 0;
    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            n += 1;
            size += path.metadata()?.len();
        } else if path.is_dir() {
            let (s, m) = n_files(&path)?;
            n += m;
            size += s;
        }
    }
    Ok((size, n))
}
