use std::env;
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let cur_dir = env::current_dir()?;
    let dir = fs::read_dir(cur_dir)?;
    let mut output = String::new();
    for entry in dir {
        match entry {
            Ok(e) => {
                let file_name = e.file_name();
                let mut file_name_str = String::new();
                if let Ok(v) = file_name.into_string() {
                    file_name_str = v;
                }
                if file_name_str.is_empty() {
                    continue;
                }
                output.push_str(&file_name_str);
                output.push('\t');
            },
            Err(e) => {
                output.push_str(format!("{:?}", e).as_str());
                output.push('\n');
                output.push_str("\n");
            },
        }
    }
    println!("{}", output);
    Ok(())
}
