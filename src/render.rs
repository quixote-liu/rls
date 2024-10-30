use std::cmp::Ordering;
use std::io::Write;
use std::{env, fs};
use crate::entry::Entry;
use crate::error;

#[derive(Default)]
pub struct Render {
    entries: Vec<Entry>,
    sub_options: Vec<SubOption>,

    max_user_len: i32,
    max_group_len: i32,
    max_size_len: i32,
    max_size_format_len: i32,
    max_count_len: i32,
}

#[derive(PartialEq, Eq, Clone)]
enum SubOption {
    Time,
    All,
    Long,
    HumanRead,
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
                    'h' => { sub_opt = Some(SubOption::HumanRead) },
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
            ..Default::default()
        }
    }

    pub fn load_file_info(&mut self) {
        self.entries.iter_mut().for_each(|e| {e.load_info()});
    }

    pub fn extract_max_info_for_entries(&mut self) {
        self.entries.iter_mut().for_each(|e| {
            let ul = e.user_name.len() as i32;
            if ul > self.max_user_len { self.max_user_len = ul };
            
            let gl = e.group_name.len() as i32;
            if gl > self.max_group_len { self.max_group_len = gl};

            let cl = e.files_number.to_string().len() as i32;
            if cl > self.max_count_len { self.max_count_len = cl};

            let fs = e.size.to_string().len() as i32;
            if fs > self.max_size_len { self.max_size_len = fs };

            let fsf = e.size_format.len() as i32;
            if fsf > self.max_size_format_len { self.max_size_format_len = fsf };
        });
    }

    pub fn format_user_name(&self, user: String) -> String {
        let mut u = user.clone();
        let diff = self.max_size_len - user.len() as i32;
        if diff > 0 {
            for _i in 0..diff { u.push_str(" ") }   
        }
        return u
    }

    pub fn format_group_name(&self, group: String) -> String {
        let mut g = group.clone();
        let diff = self.max_group_len - group.len() as i32;
        if diff > 0 {
            for _i in 0..diff { g.push_str(" ") };
        }
        return g
    }

    pub fn format_file_count(&self, count: i32) -> String {
        let mut cs = count.to_string();
        let diff = self.max_count_len - cs.len() as i32;
        if diff > 0 {
            for _i in 0..diff { cs.push_str(" ") };
        }
        return cs
    }

    pub fn start(&mut self) {
        self.load_file_info();
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
                SubOption::HumanRead => {
                    self.entries.iter_mut().for_each(|e| {
                        e.format_entry_size();
                    });
                }
            }
        }
        if output_long_info {
            self.extract_max_info_for_entries();
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
                let file_count = self.format_file_count(entry.files_number);
                let size = entry.get_file_size_display(self.max_size_format_len, self.max_size_len);
                let user = self.format_user_name(entry.user_name.clone());
                let group = self.format_group_name(entry.group_name.clone());
                let update_time = entry.update_time.format("%Y-%m-%d %H:%M:%S").to_string();
                let file_name = entry.file_name.clone();
                content = format!("{file_permission} {file_count} {user} {group} {size} {update_time} {file_name}");
            } else {
                content = entry.file_name.clone();
            }
            
            out.write(content.as_bytes()).unwrap();
        }
        out.write("\n".as_bytes()).unwrap();
    }
}