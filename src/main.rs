mod entrys;

fn main() {
    let dir_format = entrys::DirEntryFormat::init();
    dir_format.output();
}
