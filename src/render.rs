use std::{env, fs};
use crate::entry::Entry;
use crate::error;

pub struct Render {
    entries: Vec<Entry>,
    sub_options: Vec<SubOption>,
}

#[derive(PartialEq, Eq)]
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
                        let dir = read_dir::iter();
                        
                        // for (index, dir_entry) in read_dir.into_iter().enumerate() {
                        //     match dir_entry {
                        //         Ok(de) => {
                        //             let new_entry = Entry::new(de, index as i32, count as i32);            
                        //             entries.push(new_entry);
                        //         },
                        //         Err(e) => {
                        //             error::thrown_common_err(e.to_string());
                        //         },
                        //     }
                        // }
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

        Self {
            entries: entries,
            sub_options: sub_options,
        }
    }

    pub fn start(&self) {

    }
}