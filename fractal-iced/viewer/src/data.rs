use std::{
    fs::{create_dir, File},
    io::{BufRead, BufReader, Write},
    path::Path,
    slice::Iter,
};

use base::{DATA_DIR, DATA_FILE};

pub struct DataManager {
    list: Vec<String>,
}

impl DataManager {
    pub fn new() -> Self {
        if !Path::new(DATA_DIR).exists() {
            if create_dir(DATA_DIR).is_err() {
                println!("cannot create the directory {}", DATA_DIR);
            }
        }
        let filename = format!("{}/{}", DATA_DIR, DATA_FILE);
        let list;
        let path = Path::new(&filename);
        if !path.is_file() {
            list = vec![
                "Mandelbrot;-2.1;1.1;-1.5;1.5;100;100;1000;16;RGB0;z^2;z;z;2;".to_owned(),
                "Julia;-2;2;-2;2;100;100;100;16;RGB0;z^2-0.75+0.1234i;z;1/z;2;".to_owned(),
                "Newton;-2;2;-2;2;100;100;100;16;RGB0;z^3-1;z;3z^2;0.000000000001;".to_owned(),
            ];
        } else {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            list = reader
                .lines()
                .map(|line| line.unwrap())
                .filter(|line| line.len() > 30)
                .collect();
        }
        Self { list }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn iter(&self) -> Iter<'_, String> {
        self.list.iter()
    }

    pub fn get(&self, index: usize) -> &String {
        return &self.list[index];
    }

    pub fn push(&mut self, text: &str) -> bool {
        for line in self.list.iter() {
            if line == text {
                return false;
            }
        }
        self.list.push(text.to_owned());
        true
    }

    pub fn remove(&mut self, index: usize) {
        self.list.remove(index);
    }

    pub fn write_to_file(&self) {
        let mut file =
            File::create(format!("{}/{}", DATA_DIR, DATA_FILE)).unwrap();
        self.list.iter().for_each(|line| {
            file.write_all(line.as_bytes()).unwrap();
            file.write_all(b"\r\n").unwrap();
        });
    }
}
