use std::{
    error::Error,
    fs,
    io::{self, Read, Write},
};

fn write_func<W: io::Write>(bw: &mut W, s: &str) -> Result<(), Box<dyn Error>> {
    bw.write_all(s.as_bytes())?;
    Ok(())
}

fn call_func_to_write(path: &str, n: u32, s: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::File::create(path)?;
    let mut writer = io::BufWriter::new(file);
    writer.write_all(&n.to_le_bytes())?;
    write_func(&mut writer, s)?;
    writer.flush()?;
    Ok(())
}

fn read_func<R: io::Read>(br: &mut R) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; 4];
    br.read_exact(&mut buf)?;
    // u32로 변환
    let u32_value = u32::from_le_bytes(buf);
    println!("read_func: {u32_value}");
    Ok(())
}

fn call_func_to_read(path: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(path)?;
    // file에서 직접 읽어도 된다.
    let mut reader = io::BufReader::new(file);
    read_func(&mut reader)?;
    let mut buf = [0; 6];
    reader.read_exact(&mut buf)?;
    let s = String::from_utf8(buf.into())?;
    println!("call_func_to_read: {s}");

    reader.seek_relative(1)?; // 1 byte 건너뛰기
    reader.read_exact(&mut buf)?;
    let s = String::from_utf8(buf.into())?;
    println!("                   {s}");
    Ok(())
}

#[allow(dead_code)]
fn read_all(path:&str)  -> Result<(), Box<dyn Error>> {    
    let mut file = fs::File::open(path)?;
    let mut buf = Vec::new();
    let size = file.read_to_end(&mut buf)?;
    println!("read_all: {size} bytes");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "../demo135-binary-file/data.bin";
    call_func_to_write(path, 123, "한글 English")?;
    call_func_to_read(path)?;
    read_all(path)?;
    Ok(())
}
