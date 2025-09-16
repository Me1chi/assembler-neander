use std::fs::{OpenOptions};
use std::io::{BufReader, BufWriter};

const COMMENT_CHAR: char = ';';

pub fn trim_comment(s: &mut String) {
    let index = s.find(COMMENT_CHAR);

    if let Some(i) = index {
        s.truncate(i);
    }
}

pub fn open_read(path: &str) -> std::io::Result<BufReader<std::fs::File>> {
    Ok(BufReader::new(
        OpenOptions::new()
            .read(true)
            .open(path)?
    ))
}

pub fn open_write(path: &str) -> std::io::Result<BufWriter<std::fs::File>> {
    Ok(BufWriter::new(
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?
    ))
}

