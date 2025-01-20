use std::{fs::read_dir, path::{Path, PathBuf}};
use anyhow::{Result, anyhow};

use crate::info::Info;

pub struct Dir {
    path: PathBuf,
}

impl Dir {
    pub fn validate(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.is_dir() {
            return Err(anyhow!("Directory {} doesn't exist", path.display()));
        }
        Ok(Self {
            path: path.to_path_buf(),
        }) 
    }

    pub fn info(&self) -> Result<Info> {
        Ok(Info::new(n_files(&self.path)?))
    }
}

fn n_files(path: &Path) -> Result<usize> {
    let mut out = 0;
    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            out += 1;
        } else if path.is_dir() {
            out += n_files(&path)?;
        }
    }
    Ok(out)
}
