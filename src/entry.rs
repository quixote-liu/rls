use std::{
    os::unix::fs::{FileTypeExt, MetadataExt},
    path::PathBuf,
};
use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct Entry {
    path: PathBuf,
    pub is_cur: bool,
    pub file_name: String,
    pub display: bool,
    pub size: u64,
    pub size_format: String,
    pub file_permission: String,
    pub files_number: i32,
    pub user_name: String,
    pub group_name: String,
    pub update_time: DateTime<Utc>,
}

impl Entry {
    pub fn new(path: PathBuf) -> Self {
        let mut file_name = String::new();
        if let Some(v) = path.file_name() {
            if let Some(vv) = v.to_str() {
                file_name = vv.to_string();
            }
        }

        let mut display = true;
        if file_name.starts_with(".") {
            display = false;
        }

        Self {
            path,
            file_name: file_name,
            display: display,
            ..Default::default()
        }
    }

    pub fn from_cur(path: PathBuf) -> Self {
        Self {
            path,
            file_name: ".".to_string(),
            display: true,
            is_cur: true,
            ..Default::default()
        }
    }

    pub fn from_parent(path: PathBuf) -> Self {
        Self {
            path,
            file_name: "..".to_string(),
            display: true,
            ..Default::default()
        }
    }

    pub fn get_modified_time(&self) -> Option<std::time::SystemTime> {
        if let Ok(metadata) = self.path.metadata() {
            if let Ok(mtime) = metadata.modified() {
                return Some(mtime);
            }
        }
        return None;
    } 

    pub fn load_info(&mut self) {
        if let Ok(metadata) = self.path.metadata() {
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
            if self.path.is_dir() {
                count = 0;
                self.path.read_dir().unwrap().for_each(|_| count += 1);
            }
            self.files_number = count;

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

    pub fn format_entry_size(&mut self) {
        let units = ["B", "K", "M", "G", "T"];
        let mut size = self.size as f64;
        let mut i = 0;
        loop {
            if i + 1 == units.len() {
                break;
            }
            if size > 1024.0 {
                size /= 1024.0;
                i += 1;
            } else {
                break;
            }
        }
        if i == 0 {
            self.size_format = format!("{}{}", size, units[i]);
        } else {
            self.size_format = format!("{:.1}{}", size, units[i]);
        }
    }

    pub fn get_file_size_display(&self, max_size_format: i32, max_len: i32) -> String {
        let mut pad = String::new();
        if !self.size_format.is_empty() {
            let diff = max_size_format - self.size_format.len() as i32;
            for _i in 0..diff {pad.push_str(" ")};
            pad.push_str(&self.size_format.clone());
        } else {
            let size_str = self.size.to_string();
            let diff = max_len - size_str.len() as i32;
            for _i in 0..diff {pad.push_str(" ")};
            pad.push_str(&size_str);
        }
        pad
    }
}
