mod render;
mod entry;

fn main() {
    let dir_format = render::DirEntryFormat::init();
    dir_format.output();
}
