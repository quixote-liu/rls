use std::{fmt::Write, fs};

pub struct Entry {
    dir_entry: fs::DirEntry,
    front_entry: Box<Option<Entry>>,
    next_entry: Box<Option<Entry>>,
    file_name: String,

    prefix: String,
    content: String,
    suffix: String,
    // colour: String,
}

impl Entry {
    pub fn new(dir_entry :fs::DirEntry, front_entry: Option<Entry>, next_entry: Option<Entry>) -> Self {
        let mut prefix = String::new();
        if front_entry.is_some() {
            prefix = "\t".to_string();
        }
        let mut suffix = String::new();
        if next_entry.is_none() {
            suffix = "\n".to_string();
        }

        let mut file_name = String::new();
        if let Ok(v) = dir_entry.file_name().into_string() {
            file_name = v;
        }

        Self{
            dir_entry: dir_entry,
            front_entry: Box::new(front_entry),
            next_entry: Box::new(next_entry),
            file_name,
            prefix,
            content: file_name,
            suffix,
        }
    }

    pub fn set_next_entry(&self, next_entry: Option<Entry>) {
        self.next_entry = Box::new(next_entry);
    }

    pub fn set_front_entry(&self, front_entry: Option<Entry>) {
        self.front_entry = Box::new(front_entry);
    }

    pub fn render(&self) -> String {
        let mut r = String::new();
        let _ = r.write_str(&self.prefix);
        let _ = r.write_str(&self.content);
        let _ = r.write_str(&self.suffix);
        r
    }
}

