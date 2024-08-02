use std::fs;

pub struct Entry {
    dir_entry: fs::DirEntry,
}

impl Entry {
    pub fn from(e :fs::DirEntry) -> Self {
        Self{
            dir_entry: e,
        }
    }
}

