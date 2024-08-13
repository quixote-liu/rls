use std::{fmt::Write, fs};

pub struct Entry {
    dir_entry: fs::DirEntry,
    file_name: String,
    index: i32,

    prefix: String,
    content: String,
    suffix: String,
    is_display: bool,
}

impl Entry {
    pub fn new(dir_entry :fs::DirEntry, index: i32, total: i32) -> Self {
        let mut prefix = String::new();
        if index > 0 {
            prefix = "\t".to_string();
        }
        let mut suffix = String::new();
        if index+1 == total {
            suffix = "\n".to_string();
        }

        let mut file_name = String::new();
        if let Ok(v) = dir_entry.file_name().into_string() {
            file_name = v;
        }

        Self{
            dir_entry: dir_entry,
            file_name,
            index,
            prefix,
            content: file_name,
            suffix,
            is_display: false,
        }
    }

    pub fn render(&self) -> String {
        let mut r = String::new();
        let _ = r.write_str(&self.prefix);
        let _ = r.write_str(&self.content);
        let _ = r.write_str(&self.suffix);
        r
    }
}

