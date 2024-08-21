use std::cmp::Ordering;
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

        let mut render = Self{
            entries: entries,
            sub_options: sub_options,
        };

        render.set_entries_index();

        render
    }

    fn set_entries_index(&mut self) {
        // init entries attributes
        let count = self.entries.len();
        for (i, e) in self.entries.iter_mut().enumerate() {
            if i == 0 {
                e.is_first(true)
            }
            if i + 1 == count {
                e.is_last(true)
            }
            e.set_index(i as i32);
        }
    }

    pub fn start(&mut self) {
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
                },
                SubOption::Time => {
                    self.entries.sort_by(|a, b| {
                        if let (Ok(m1), Ok(m2)) = (a.dir_entry().metadata(), b.dir_entry().metadata()) {
                            if let (Ok(t1), Ok(t2)) = (m1.modified(), m2.modified()) {
                                return t1.cmp(&t2)
                            }
                        }
                        Ordering::Equal
                    });
                    self.set_entries_index();
                },
            }
        }
    }
}