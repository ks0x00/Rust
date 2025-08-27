use std::{
    error::Error,
    fs,
    io::{self, BufRead, Write},
};

#[allow(dead_code)]
// read a line from a file with encoding utf-8
fn read_line_by_line(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let mut reader = io::BufReader::new(file);
    let mut lines = Vec::new();
    println!("read_line_by_line:");
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(len) => {
                // check end of file
                if len == 0 {
                    break;
                }
                // remove end of line characters
                // line = line.trim()?;
                if line.ends_with("\r\n") {
                    line = line[..len - 2].to_owned();
                }
                else if line.ends_with("\n") {
                    line = line[..len - 1].to_owned();
                }
                println!("    {line}, length: {len}");
                lines.push(line);
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(lines)
}

#[allow(dead_code)]
// read all lines from a file with encoding utf-8
fn read_all_lines(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<String> = content.lines().map(String::from).collect();
    println!("read_all_lines: {:?}", lines);
    Ok(lines)
}

#[allow(dead_code)]
fn create_and_write(path: &str, s: &str) -> Result<(), Box<dyn Error>> {
    fs::write(path, s)?;
    Ok(())
}

#[allow(dead_code)]
fn create_or_append(path: &str, s: &str) -> Result<(), Box<dyn Error>> {
    let mut file = fs::OpenOptions::new()
        // .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(s.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "../demo130-text-file-utf8/data.txt";
    create_and_write(path, "첫째 줄\n둘째 줄")?;
    create_or_append(path, "\n셋째 줄")?;
    read_line_by_line(path)?;
    read_all_lines(path)?;
    Ok(())
}