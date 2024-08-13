mod render;
mod entry;
mod error;

fn main() {
    let dir_format = render::DirEntryFormat::init();
    dir_format.output();
}
