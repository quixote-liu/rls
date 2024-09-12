use std::{fmt::Write, fs, os::unix::fs::{FileTypeExt, MetadataExt}};

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
        // file metadata info
        let mut file_type = String::new();
        
        if let Ok(metadata) = self.dir_entry.metadata() {
            let ft = metadata.file_type();
            let mut ft_flag = " ";
            if ft.is_dir() {
                ft_flag = "d";
            } else if ft.is_file() {
                ft_flag = "-";
            } else if ft.is_symlink() {
                ft_flag = "l";
            } else if ft.is_block_device() {
                ft_flag = "b";
            } else if ft.is_char_device() {
                ft_flag = "c";
            } else if ft.is_socket() {
                ft_flag = "s";
            } else if ft.is_fifo() {
                ft_flag = "p";
            }
            file_type.push_str(ft_flag);

            let mode = metadata.mode();
            let o_mode = format!("{:b}", mode);
            let mut file_permission = String::new();
            let mut per_flag = "rwx";
            for ele in o_mode.chars().into_iter() {
                if file_permission.len() >= 9 {
                    break;
                }
                if ele == '1' {
                    let i = file_permission.len()%3;
                    let chars: Vec<char> = per_flag.chars().collect();
                    if let Some(c) = chars.get(i) {
                        file_permission.push(c.clone());
                    }
                } else {
                    file_permission.push_str("-");
                }
            }
            file_type.push_str(&file_permission);
        }
        // permissions
    }

    pub fn is_last(&mut self, v: bool) {
        self.is_last = v
    }

    pub fn is_first(&mut self, v: bool) {
        self.is_first = v
    }
}

