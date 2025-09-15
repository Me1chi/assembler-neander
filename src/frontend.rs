use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

use crate::NeanderMem;

//
//  - Now THERE'RE LITERALLY ONLY INSTRUCTIONS AND NUMBERS
//      - Just parse each line to a pair (Instruction, Option<Operand>)
//
//  - Write the output.mem
//  - Done

pub const BUILD_FILE_NAME: &str = "build";
const TEMPORARY_FILE_NAME: &str = "tmp-build";
const COMMENT_CHAR: char = ';';
const LABEL_CHAR: char = ':';
const IMMEDIATE_ADDR_CHAR: char = '$';

pub enum CustomError {
    Instruction,
    Label,
    Directive,
    MalformedLine,
    NotEnoughMemory,
}

pub struct ErrorLog {
    errs: Vec<(CustomError, usize)>, // usize is used to store the line of the error
}

#[derive(Debug)]
pub struct Segment {
    addr: usize,
    len: usize,
}

impl Segment {
    fn new() -> Self {
        Segment {
            addr: 0,
            len: 0,
        }
    }
}

#[derive(Debug)]
pub struct SegInfo { // (usize, usize) means index (like in a Vec of Strings), length of the Vec
    code: Segment, // Will be the line 0 of the memory (or CODE_SEG)
    data: Segment, // Same but 0x80 (DATA_SEG)
}

impl SegInfo {
    fn new() -> Self {
        SegInfo {
            code: Segment::new(),
            data: Segment::new(),
        }
    }

    pub fn from_build(build_filename: &str) -> Result<Self, std::io::Error> {
        let build = open_read(build_filename)?;
        let mut tmp = open_write(TEMPORARY_FILE_NAME)?;

        // Initial file handling done

        let mut info = SegInfo::new();

        let mut on_code_seg = false;
        let mut on_data_seg = false;

        let mut line_counter = 0;

        for line in build.lines() {
            let line = line?;

            if line == ".code" {
                info.code.addr = line_counter;
                on_code_seg = true;
                on_data_seg = false;
                continue;
            } else if line == ".data" {
                info.data.addr = line_counter;
                on_code_seg = false;
                on_data_seg = true;
                continue;
            }

            // All below here will be written in the "next" build file

            let num = if Label::from_str_counter(&line, line_counter).is_none() {
                line.split_whitespace().count()
            } else {
                line.split_whitespace().skip(1).count()
            };

            if on_code_seg {
                info.code.len += num;
            } else if on_data_seg {
                info.data.len += num;
            }

            if on_code_seg || on_data_seg {
                line_counter += num;
            }

            if on_code_seg || on_data_seg {
                writeln!(tmp, "{}", line)?;
            }
        }

        fs::rename(TEMPORARY_FILE_NAME, build_filename)?;

        Ok(info)

    }
}

#[derive(Debug)]
pub struct Label {
    name: String,
    addr: usize,
}

impl Label {

    pub fn from_str_counter(line: &str, counter: usize) -> Option<Label> {
        let line = line.split_whitespace().next()?;
        if let Some(c) = line.chars().last() && c == LABEL_CHAR {
            let mut line_string = String::from(line);
            line_string.pop();
            Some(Label {
                name: line_string,
                addr: counter,
            })
        } else {
            None
        }
    }

    pub fn vec_from_build(seg_info: &SegInfo, mem: &NeanderMem, build_filename: &str) -> Result<Vec<Self>, std::io::Error> {
        let build = open_read(build_filename)?;
        let mut tmp = open_write(TEMPORARY_FILE_NAME)?;

        let mut labels: Vec<Label> = Vec::new();

        let mut line_counter = 0;

        for line in build.lines() {
            let line = line?;
            let mut words = line.split_whitespace();

            let mut to_write: Vec::<&str> = Vec::new();

            let first_word = words.next().expect("HERE THERE SHOULDN'T BE AN EMPTY LINE");
            if let Some(mut label) = Label::from_str_counter(first_word, line_counter) {

                if line_counter >= seg_info.code.addr && line_counter < seg_info.code.addr + seg_info.code.len {
                    label.addr -= seg_info.code.addr;
                    label.addr += mem.code_seg;
                } else if line_counter >= seg_info.data.addr && line_counter < seg_info.data.addr + seg_info.data.len {
                    label.addr -= seg_info.data.addr;
                    label.addr += mem.data_seg;
                }


                labels.push(label);
            } else {
                to_write.push(first_word);
            }

            let last_words: Vec<&str> = words.collect();

            to_write.extend(last_words);

            line_counter += to_write.len();
            let to_write: String = to_write.join(" ");
            if !to_write.is_empty() {
                    writeln!(tmp, "{}", to_write)?;
            }
        }

        fs::rename(TEMPORARY_FILE_NAME, build_filename)?;

        Ok(labels)

    }
}

#[derive(Debug)]
pub struct ImmediateAddressing {
    value: u8,
    addr: usize,
}

impl ImmediateAddressing {

    pub fn vec_from_build(last_mem_addr: usize, build_filename: &str) -> Result<Vec<Self>, std::io::Error> {
        let build = open_read(build_filename)?;
        let mut tmp = open_write(TEMPORARY_FILE_NAME)?;

        let mut addrs: Vec<ImmediateAddressing> = Vec::new();

        let mut curr_addr = last_mem_addr;
        for line in build.lines() {
            let line = line?;

            // The operand that can be immediate or not
            let mut maybe_operand = line
                .split_whitespace()
                .last()
                .expect("NO LINES SHOULD BE EMPTY AT THIS POINT")
                .chars();

            let mut new_line_vec: Vec<&str> = line
                .split_whitespace()
                .collect();

            let addr_string: String;
            if let Some(c) = maybe_operand.next() && c == IMMEDIATE_ADDR_CHAR {
                let value = maybe_operand
                    .collect::<String>()
                    .parse()
                    .expect("SHOULD BE AN 8 BIT NUMBER HERE");


                let mut is_already_added = false;
                let mut true_addr = 0;
                for item in &addrs {
                    if value == item.value {
                        is_already_added = true;
                        true_addr = item.addr;
                    }
                }

                if !is_already_added {
                    addrs.push(ImmediateAddressing {
                        value,
                        addr: curr_addr,
                    });

                    true_addr = curr_addr;
                    curr_addr -= 1;
                }
                addr_string = true_addr.to_string(); 

                new_line_vec.pop();
                new_line_vec.push(&addr_string);
            }

            let new_line = new_line_vec.join(" ");

            writeln!(tmp, "{}", new_line)?;

        }

        fs::rename(TEMPORARY_FILE_NAME, build_filename)?;

        Ok(addrs)

    }

}

#[derive(Debug)]
pub struct LabelInfo {
    labels: Vec<Label>,
    immediates: Vec<ImmediateAddressing>,
}

impl LabelInfo {

    pub fn new(info: &SegInfo, mem: &NeanderMem, last_mem_addr: usize, build_filename: &str) -> Result<Self, std::io::Error> {
        Ok(LabelInfo {
            labels: Label::vec_from_build(info, mem, build_filename)?,
            immediates: ImmediateAddressing::vec_from_build(last_mem_addr, build_filename)?,
        })
    }

    pub fn apply_to_operands(&self, build_filename: &str) -> Result<(), std::io::Error> {

        let build = open_read(build_filename)?;
        let mut tmp = open_write(TEMPORARY_FILE_NAME)?;

        for line in build.lines() {
            let line = line?;
            let maybe_label = line.clone()
                .split_whitespace()
                .last()
                .expect("THERE SHOULD BE NO EMPTY LINES")
                .to_string();

            let mut new_line_vec: Vec<&str> = line
                .split_whitespace()
                .collect();

            let true_addr: String;
            if let Some(label) = self.labels
                .iter()
                .find(|x| x.name == maybe_label) {


                true_addr = label.addr.to_string();
                new_line_vec.pop();
                new_line_vec.push(&true_addr);
            }

            let line = new_line_vec.join(" ");
            writeln!(tmp, "{}", line)?;
        }

        fs::rename(TEMPORARY_FILE_NAME, build_filename)?;

        Ok(())

    }

}

pub fn trim_comment(s: &mut String) {
    let index = s.find(COMMENT_CHAR);

    if let Some(i) = index {
        s.truncate(i);
    }
}

pub fn create_build_file(source_filename: &str, build_filename: &str) -> std::io::Result<()> {
    let src = open_read(source_filename)?;
    let mut build = open_write(build_filename)?;

    for line in src.lines() {

        let mut line = line?.to_lowercase();

        trim_comment(&mut line);
        let line = line.trim();

        if !line.is_empty() {
            writeln!(build, "{}", line)?;
        }
    }

    Ok(())
}

fn open_read(path: &str) -> std::io::Result<BufReader<std::fs::File>> {
    Ok(BufReader::new(
        OpenOptions::new()
            .read(true)
            .open(path)?
    ))
}

fn open_write(path: &str) -> std::io::Result<BufWriter<std::fs::File>> {
    Ok(BufWriter::new(
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?
    ))
}




