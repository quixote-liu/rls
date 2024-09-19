use std::{fmt::{Write}, fs, os::unix::fs::{FileTypeExt, MetadataExt}};

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
        let mut file_type = String::new();

        if let Ok(metadata) = self.dir_entry.metadata() {
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
            file_type.push_str(ft_flag);

            // permissions
            let mut mode_bit_raw_str = format!("{:b}", metadata.mode());
            let mut file_permission = String::from("");
            if mode_bit_raw_str.len() >= 9 {
                let permission_model = "rwxrwxrwx";
                let mode_bit_str = mode_bit_raw_str.split_off(mode_bit_raw_str.len()-9);
                for (i, v) in mode_bit_str.as_bytes().iter().enumerate() {
                    if *v == b'1' {
                        file_permission.push_str(&permission_model[i].clone().to_string());
                    } else {
                        file_permission.push_str("-");
                    }
                }
            }
            if file_permission.len() == 0 {
                file_permission = "---------".to_string();
            }
            file_type.push_str(&file_permission);
        }

        self.content = format!("{}  {}", file_type, self.file_name);
    }

    pub fn is_last(&mut self, v: bool) {
        self.is_last = v
    }

    pub fn is_first(&mut self, v: bool) {
        self.is_first = v
    }
}

// fn extract_permissions_from_mode(mode: u32) -> &str {
//     let mode_o = format!("{:o}", mode);
//     let mut trans_i = 0;
//     for i in 0..mode_o.len() {
//         if mode_o.len() - i - 1 == 9 {
//             trans_i = i;
//             break;
//         }
//     }
//     for i in trans_i..mode_o.len() {
        
//     }
//     let mode_vec: Vec<u8> = mode.as_bytes().to_vec();
    
// }

