use anyhow::Result;
use std::{collections::HashSet, fs::ReadDir, os::linux::fs::MetadataExt, path::Path};

macro_rules! try_or_return {
    ($x:expr) => {{
        if let Err(e) = $x {
            return Some(Err(e));
        }
        $x.unwrap()
    }};
}

pub struct DirWalker {
    read_dir_stack: Vec<ReadDir>,
    visited_ids: HashSet<FileId>,
}

impl DirWalker {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            read_dir_stack: vec![path.as_ref().read_dir()?],
            visited_ids: HashSet::new(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct FileId {
    inode: u64,
    device: u64,
}

impl FileId {
    fn new(inode: u64, device: u64) -> Self {
        Self { inode, device }
    }
}

pub struct FileEntry {
    size: u64,
}

impl FileEntry {
    fn new(size: u64) -> Self {
        Self { size }
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

impl Iterator for DirWalker {
    type Item = Result<FileEntry, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(read_dir) = self.read_dir_stack.last_mut() {
            let mut early_break = false;
            for entry in read_dir {
                let entry = try_or_return!(entry);
                let md = try_or_return!(entry.metadata());

                if md.is_symlink() {
                    continue; //ignore symlinks
                } else if md.is_dir() {
                    let new_read_dir = try_or_return!(entry.path().read_dir());
                    self.read_dir_stack.push(new_read_dir);
                    early_break = true;
                    break;
                }

                let f_id = FileId::new(md.st_ino(), md.st_dev());
                if self.visited_ids.contains(&f_id) {
                    continue;
                }
                self.visited_ids.insert(f_id);

                let f_entry = FileEntry::new(md.len());
                return Some(Ok(f_entry));
            }
            if !early_break {
                self.read_dir_stack.pop();
            }
        }
        None
    }
}
