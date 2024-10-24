use std::{
    fs, os::unix::fs::{FileTypeExt, MetadataExt}
};
use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct Entry {
    entry: Option<fs::DirEntry>,
    pub file_name: String,
    pub display: bool,
    pub size: u64,
    pub file_permission: String,
    pub files_number: String,
    pub user_name: String,
    pub group_name: String,
    pub update_time: DateTime<Utc>,
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
        Self {
            entry: Some(dir_entry),
            file_name: file_name,
            display: display,
            ..Default::default()
        }
    }

    pub fn get_modified_time(&self) -> Option<std::time::SystemTime> {
        if let Some(ref entry) = self.entry {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(mtime) = metadata.modified() {
                    return Some(mtime);
                }
            }
        }
        return None;
    } 

    pub fn load_info(&mut self) {
        if let Some(ref entry) = self.entry {
            if let Ok(metadata) = entry.metadata() {
                // file metadata info
                let ft = metadata.file_type();
                let mut ft_flag = "-";
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
                self.file_permission.push_str(&ft_flag);
    
                // permissions
                let mut raw_mode_bit = format!("{:b}", metadata.mode());
                let mut file_permission = String::from("");
                if raw_mode_bit.len() >= 9 {
                    let permission_model = "rwxrwxrwx";
                    let mode_bit = raw_mode_bit.split_off(raw_mode_bit.len()-9);
                    for (i, c) in mode_bit.chars().into_iter().enumerate() {
                        if c == '1' {
                            file_permission.push_str(&permission_model[i..i+1]);
                        } else {
                            file_permission.push_str("-");
                        }
                    }
                }
                if file_permission.len() == 0 {
                    file_permission = ["-"; 9].join("");
                }
                self.file_permission.push_str(&file_permission);
    
                // get files number
                let mut count = 1;
                let file_path = entry.path();
                if file_path.is_dir() {
                    count = 0;
                    file_path.read_dir().unwrap().for_each(|_| count += 1);
                }
                self.files_number = count.to_string();
    
                // user and user-group infomation
                // TODO: transform user id to user name
                let uid = metadata.uid();
                let gid = metadata.gid();
                self.user_name = uid.to_string();
                self.group_name = gid.to_string();
    
                // set file size
                let size = metadata.size();
                self.size = size;
    
                // set update time
                let mtime = metadata.mtime();
                if let Some(t) = DateTime::from_timestamp(mtime, 0) {
                    self.update_time = t;
                }
            }
        }
    }
}
