use std::{error::Error, fs, io::BufRead};

use encoding_rs::EUC_KR;

fn decode_euckr(buf: &[u8]) -> String {
    let (cow, _, had_errors) = EUC_KR.decode(&buf);
    if had_errors {
        eprintln!("경고: 디코딩 중 오류가 발생하여 대체 문자로 변환된 부분이 있습니다.");
    }
    cow.to_string()
}

fn read_2_lines(path: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);

    reader.skip_until(b'\n')?; // 첫 번째 줄 건너뛰기

    let mut buf = Vec::new();

    reader.read_until(b'\n', &mut buf)?;
    let content = decode_euckr(&buf);
    println!("{}", content.trim());

    buf.clear();
    reader.read_until(b'\n', &mut buf)?;
    let (cow, _, had_errors) = EUC_KR.decode(&buf);
    if had_errors {
        eprintln!("경고: 디코딩 중 오류가 발생하여 대체 문자로 변환된 부분이 있습니다.");
    }
    println!("{}", cow.to_string().trim());

    Ok(())
}

fn read(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // 1. 파일을 바이너리(바이트)로 읽기
    let bytes = fs::read(path)?;

    // 2. euc-kr → UTF-8 디코딩
    let (cow, _, had_errors) = EUC_KR.decode(&bytes);

    if had_errors {
        eprintln!("경고: 디코딩 중 오류가 발생하여 대체 문자로 변환된 부분이 있습니다.");
    }

    // 3. Cow<str> → String 변환
    let content = cow.to_string();
    let lines = content.lines().map(String::from).collect();
    Ok(lines)
}

fn write(path: &str, s: &str) -> Result<(), Box<dyn Error>> {
    let (cow, _, had_errors) = EUC_KR.encode(s);

    if had_errors {
        eprintln!("경고: 인코딩 중 일부 문자가 대체 문자로 변환되었습니다.");
        // return Err("EUC-KR로 인코딩할 수 없는 문자가 포함되어 있습니다.".into());
    }

    fs::write(path, cow.as_ref())?;
    Ok(())
}

/// 🔹 Vec<String> 뿐만 아니라 &["..."] 같은 슬라이스도 받을 수 있도록 개선
fn write_lines<S: AsRef<str>>(path: &str, lines: &[S]) -> Result<(), Box<dyn Error>> {
    let content = lines
        .iter()
        .map(|s| s.as_ref())
        .collect::<Vec<_>>()
        .join("\n");
    write(path, &content)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "../demo131-text-file-euc_kr/data.txt";
    let content = ["안녕", "여긴 어디?", "나는 누구?"];
    write_lines(path, &content)?;

    read_2_lines(path)?;
    println!("------------------");

    for line in read(path)? {
        println!("{line}");
    }
    Ok(())
}
