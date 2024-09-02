use std::{fmt::Write, fs};

pub struct Entry {
    dir_entry: fs::DirEntry,
    pub file_name: String,

    prefix: String,
    content: String,
    suffix: String,
    display: bool,
    is_last: bool,
    is_first: bool,
}

impl Entry {
    pub fn new(dir_entry :fs::DirEntry) -> Self {
        let mut file_name = String::new();
        if let Ok(v) = dir_entry.file_name().into_string() {
            file_name = v;
        }
        let mut display = true;
        if file_name.starts_with(".") {
            display = false;
        }
        Self{
            dir_entry: dir_entry,
            file_name: file_name.clone(),
            prefix: "".to_string(),
            content: file_name,
            suffix: "".to_string(),
            display,
            is_first: false,
            is_last: false,
        }
    }

    pub fn render(&self) -> String {
        if !self.display {
            return "".to_string();
        }
        let mut r = String::new();
        let (prefix, suffix) = self.get_prefix_and_suffix();
        let _ = r.write_str(&prefix);
        let _ = r.write_str(&self.content);
        let _ = r.write_str(&suffix);
        r
    }

    fn get_prefix_and_suffix(&self) -> (String, String) {
        let mut prefix = self.prefix.clone();
        if prefix.is_empty() && !self.is_first {
            prefix.push_str("  ");
        }
        let mut suffix = self.suffix.clone();
        if suffix.is_empty() && self.is_last {
            suffix.push_str("\n");
        }
        (prefix, suffix)
    }

    pub fn dir_entry(&self) -> &fs::DirEntry {
        &self.dir_entry
    }

    pub fn set_display(&mut self, flag: bool) {
        self.display = flag
    }

    pub fn get_display(&self) -> bool {
        self.display
    }

    pub fn set_long_info(&mut self) {
        
    }

    pub fn is_last(&mut self, v: bool) {
        self.is_last = v
    }

    pub fn is_first(&mut self, v: bool) {
        self.is_first = v
    }
}

