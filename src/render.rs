use std::cmp::Ordering;
use std::io::Write;
use std::{env, fs};
use crate::entry::Entry;
use crate::error;

pub struct Render {
    entries: Vec<Entry>,
    sub_options: Vec<SubOption>,
}

#[derive(PartialEq, Eq, Clone)]
enum SubOption {
    Time,
    All,
    Long,
}

impl Render {
    pub fn init() -> Self {
        let mut sub_options = Vec::new();
        let args: Vec<String> = env::args().collect();
        for arg in args {
            if !arg.starts_with("-") || arg.starts_with("--") {
                continue;
            }
            let a = arg.trim_start_matches("-");
            for c in a.chars() {
                let mut sub_opt: Option<SubOption> = None;
                match c {
                    'a' => { sub_opt = Some(SubOption::All) },
                    't' => { sub_opt = Some(SubOption::Time) },
                    'l' => { sub_opt = Some(SubOption::Long) },
                    _ => { error::thrown_subopt_err(c.to_string(), "not support".to_string()) },
                }
                if let Some(sub_opt) = sub_opt {
                    if sub_options.contains(&sub_opt) {
                        error::thrown_subopt_err(c.to_string(), "duplicated".to_string());
                    } else {
                        sub_options.push(sub_opt);
                    }
                }
            }
        };

        let mut entries = Vec::new();
        match env::current_dir() {
            Ok(dir) => {
                match fs::read_dir(dir) {
                    Ok(read_dir) => {
                        for dir_entry in read_dir.into_iter() {
                            match dir_entry {
                                Ok(de) => {
                                    entries.push(Entry::new(de));
                                },
                                Err(e) => {
                                    error::thrown_common_err(e.to_string());
                                },
                            }
                        }
                    },
                    Err(e) => {
                        error::thrown_common_err(e.to_string());
                    },
                }
            },
            Err(e) => {
                error::thrown_common_err(e.to_string());
            },
        }

        Self{
            entries: entries,
            sub_options: sub_options,
        }
    }

    pub fn init_entries_info(&mut self) {
        let mut user_len = 0;
        let mut group_len= 0;
        let mut file_num_len = 0;
        self.entries.iter_mut().for_each(|e| {
            e.load_info();
            
            let ul = e.user_name.len() as i32;
            if ul > user_len { user_len = ul };
            
            let gl = e.group_name.len() as i32;
            if gl > group_len { group_len = gl};

            let cl = e.files_number.len() as i32;
            if cl > file_num_len { file_num_len = cl};
        });
        self.entries.iter_mut().for_each(|e| {
            let u_diff = user_len - e.user_name.len() as i32;
            if u_diff > 0 {
                let mut added = "".to_string();
                for _i in 0..u_diff { added.push(' ');}
                e.user_name.push_str(&added);
            }
            let g_diff = group_len - e.group_name.len() as i32;
            if g_diff > 0 {
                let mut added = "".to_string();
                for _i in 0..g_diff { added.push(' ') }
                e.group_name.push_str(&added);
            }
            let c_diff = file_num_len - e.files_number.len() as i32;
            if c_diff > 0 {
                let mut added = "".to_string();
                for _i in 0..c_diff {added.push(' ')};
                e.files_number.push_str(&added);
            }
        });
    }

    pub fn start(&mut self) {
        self.init_entries_info();
        
        let mut output_long_info = false;
        for sub_opt in self.sub_options.clone() {
            match sub_opt {
                SubOption::All => {
                    self.entries.iter_mut().for_each(|e| {
                        e.display = true;
                    });
                },
                SubOption::Long => {
                    output_long_info = true;
                },
                SubOption::Time => {
                    self.entries.sort_by(|a, b| {
                        if let (Some(m1), Some(m2)) = (a.get_modified_time(), b.get_modified_time()) {
                            return m2.cmp(&m1);
                        }
                        Ordering::Equal
                    });
                },
            }
        }
        let mut out = std::io::stdout();
        for (i, entry) in self.entries.iter().enumerate() {
            if !entry.display {
                continue;
            }
            let prefix;
            if i == 0 {
                prefix = "".to_string();
            } else if output_long_info {
                prefix = "\n".to_string();
            } else {
                prefix = "  ".to_string();
            }
            out.write(prefix.as_bytes()).unwrap();

            let content;
            if output_long_info {
                let file_permission = entry.file_permission.clone();
                let file_count = entry.files_number.clone();
                let user = entry.user_name.clone();
                let group = entry.group_name.clone();
                let update_time = entry.update_time.format("%Y-%m-%d %H:%M:%S").to_string(); // TODO: optimize
                let file_name = entry.file_name.clone();
                content = format!("{file_permission} {file_count} {user} {group} {update_time} {file_name}");
            } else {
                content = entry.file_name.clone();
            }
            
            out.write(content.as_bytes()).unwrap();
        }
        out.write("\n".as_bytes()).unwrap();
    }
}