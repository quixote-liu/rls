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

    pub fn start(&mut self) {
        let mut change_line_output = false;
        for sub_opt in self.sub_options.clone() {
            match sub_opt {
                SubOption::All => {
                    self.entries.iter_mut().for_each(|e| {
                        e.set_display(true);
                    });
                },
                SubOption::Long => {
                    self.entries.iter_mut().for_each(|e| {
                        e.set_long_info();
                    });
                    change_line_output = true;
                },
                SubOption::Time => {
                    self.entries.sort_by(|a, b| {
                        if let (Ok(m1), Ok(m2)) = (a.dir_entry().metadata(), b.dir_entry().metadata()) {
                            if let (Ok(t1), Ok(t2)) = (m1.modified(), m2.modified()) {
                                return t2.cmp(&t1)
                            }
                        }
                        Ordering::Equal
                    });
                },
            }
        }
        let mut out = std::io::stdout();
        for (i, entry) in self.entries.iter().enumerate() {
            if !entry.is_display() {
                continue;
            }
            let prefix;
            if i == 0 {
                prefix = "".to_string();
            } else if change_line_output {
                prefix = "\n".to_string();
            } else {
                prefix = "  ".to_string();
            }
            out.write(prefix.as_bytes()).unwrap();

            out.write(entry.content().as_bytes()).unwrap();
        }
        out.write("\n".as_bytes()).unwrap();
    }
}