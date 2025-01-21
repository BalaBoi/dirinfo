use anyhow::{anyhow, Result};
use std::{
    collections::HashSet, fs::read_dir, os::linux::fs::MetadataExt, path::{Path, PathBuf}
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
        let (size, n_files) = n_files(&self.path, &mut HashSet::new())?;
        Ok(Info::new(size, n_files))
    }
}

fn n_files(path: &Path, set: &mut HashSet<FileEntry>) -> Result<(u64, usize)> {
    let mut n = 0;
    let mut size = path.metadata()?.st_blocks() * 512;
    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;
        let f_entry = FileEntry {
            inode: metadata.st_ino(),
            device: metadata.st_dev(),
        };
        if set.contains(&f_entry) {
            continue;
        } else {
            set.insert(f_entry);
        }
        if path.is_file() {
            n += 1;
            size += metadata.st_blocks() * 512;
        } else if path.is_dir() {
            let (s, m) = n_files(&path, set)?;
            n += m;
            size += s + metadata.st_blocks() * 512;
        }
    }
    Ok((size, n))
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct FileEntry {
    inode: u64,
    device: u64,
}
