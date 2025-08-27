use std::{
    fs,
    io::{self, Write},
    path,
};

fn main() {
    let path = path::Path::new(".");
    checkdir(&path);
    print!("Press Enter ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
}
fn checkdir(path: &path::Path) {
    for entry in fs::read_dir(path).unwrap() {
        let entry_path = entry.unwrap().path();
        if entry_path.is_dir() {
            let filename = entry_path.file_name().unwrap().to_str().unwrap();
            if filename == "target" {
                fs::remove_dir_all(&entry_path).unwrap();
                println!("{}", entry_path.to_str().unwrap());
            } else {
                checkdir(&entry_path);
            }
        }
    }
}
