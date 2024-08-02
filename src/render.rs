use std::env;
use std::fs;
use std::path;
use crate::entry::Entry;

pub struct Render {
    entrys: Box<Vec<Entry>>,
    error: std::io::Error,
}

impl Render {
    pub fn new() -> Self {


        let entrys = Vec::from());
        
        Self{
            entrys: Box::new(),
        }
    }
}

#[derive(Default)]
pub struct DirEntryFormat {
    entrys: Vec<fs::DirEntry>,
    entry_errs: Option<Vec<std::io::Error>>,
    other_err: Option<std::io::Error>,
}

// enum SubOption {
//     All,
//     Time,
//     Help,
// }

impl DirEntryFormat {
    pub fn init() -> Self {
        let mut dir_entry_format = DirEntryFormat::default();

        let cur_dir_path: path::PathBuf;
        match env::current_dir() {
            Ok(path) => {cur_dir_path = path},
            Err(e) => {
                dir_entry_format.other_err = Some(e);
                return dir_entry_format;
            },
        }

        let mut entry_errs: Vec<std::io::Error> = Vec::new();
        match fs::read_dir(cur_dir_path) {
            Ok(dir) => {
                for entry in dir {
                    match entry {
                        Ok(entry)=>{dir_entry_format.entrys.push(entry)},
                        Err(e) =>{ entry_errs.push(e) },
                    }
                }
            },
            Err(e) => {
                dir_entry_format.other_err = Some(e);
                return dir_entry_format;
            },
        }
        if entry_errs.len() > 0 {
            dir_entry_format.entry_errs = Some(entry_errs);
        }

        dir_entry_format
    }

    pub fn output(&self) {
        if let Some(err) = &self.other_err {
            println!("{}", err);
            return;
        }

        let mut res = String::new();
        for e in &self.entrys {
            let file_name = e.file_name();
            let mut file_name_str = String::new();
            if let Ok(v) = file_name.into_string() {
                file_name_str = v;
            }
            if file_name_str.is_empty() || file_name_str.starts_with(".") {
                continue;
            }
            res.push_str(&file_name_str);
            res.push('\t');
        }

        println!("{}", res);
    }
}