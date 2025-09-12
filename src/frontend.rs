use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

// The goal of this part of the code is to:
//  - First, remove all the comments
//  - Remove all the trailling whitespaces and blank lines
//      - Save the file (build).
//
//  - Open the new build file
//      - Look for wrong things, these are:
//          - Wrong instructions
//          - Wrong directives
//          - Wrong labels
//          - Inconscistent operands
//          - Too many labels or immediate addressings
//          - Two things in the same line (e.g. A label and a directive)
//          - Report them and their lines to the user
//
//  - NOW THE FILE IS PERFECTLY WRITTEN
//
//  - Track where .code and .data start AND HOW MANY WORDS THERE'RE "INSIDE" THEM
//  - Delete their directives
//
//  - Remove all the \n's for whitespaces
//
//  - Put all the label lines in place:
//      - How? Iterate through the entire file incrementing
//              a counter that will be push into the Vec.
//              - When a label is found:
//                  - Push a pair containing the name (label itself)
//                      and the line it represents.
//                  - DELETE IT.
//
//              - Now when a label inside a instruction is found:
//                  - Iterate through the vector and find the
//                      corresponding line name. Change the label
//                      to it
//
//  - NOW THERE'RE ONLY INSTRUCTIONS, NUMBERS ANNNNND '$'.
//
//  - Yeah, find all the occurences of the $x (where x is an 8-bit number)
//      - For EACH, test if the following exists, and, if it does,
//          just "point" into it.
//      - Assign (backwards) an address for each different number found,
//      push its address and literal value to a Vec.
//
//  - Now THERE'RE LITERALLY ONLY INSTRUCTIONS AND NUMBERS
//      - Just parse each line to a pair (Instruction, Option<Operand>)
//
//  - Write the output.mem
//  - Done

const BUILD_FILE_NAME: &str = "build";
const TEMPORARY_FILE_NAME: &str = "tmp-build";
const COMMENT_CHAR: char = ';';
const LABEL_CHAR: char = ':';

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
pub struct SegInfo { // (usize, usize) means index (like in a Vec of Strings), length of the Vec
    code: (usize, usize), // Will be the line 0 of the memory (or CODE_SEG)
    data: (usize, usize), // Same but 0x80 (DATA_SEG)
}

impl SegInfo {
    fn new() -> Self {
        SegInfo {
            code: (0, 0),
            data: (0, 0),
        }
    }

    pub fn from_build() -> Result<Self, std::io::Error> {
        let build_file = OpenOptions::new()
            .write(false)
            .read(true)
            .open(BUILD_FILE_NAME)?;

        let tmp_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(TEMPORARY_FILE_NAME)?;

        let build = BufReader::new(build_file);
        let mut tmp = BufWriter::new(tmp_file);

        // Initial file handling done

        let mut info = SegInfo::new();

        let mut on_code_seg = false;
        let mut on_data_seg = false;

        let mut line_counter = 0;

        for line in build.lines() {
            let line = line?;

            if line == ".code" {
                info.code.0 = line_counter;
                on_code_seg = true;
                on_data_seg = false;
                continue;
            } else if line == ".data" {
                info.data.0 = line_counter;
                on_code_seg = false;
                on_data_seg = true;
                continue;
            }

            // All below here will be written in the "next" build file

            if !is_label(&line) {
                let num = line.split_whitespace().count();

                if on_code_seg {
                    info.code.1 += num;
                } else if on_data_seg {
                    info.data.1 += num;
                }

                if on_code_seg || on_data_seg {
                    line_counter += num;
                }
            }

            writeln!(tmp, "{}", line)?;
        }

        fs::rename(TEMPORARY_FILE_NAME, BUILD_FILE_NAME)?;

        Ok(info)

    }
}

pub fn is_label(line: &str) -> bool {
    if let Some(c) = line.chars().last() && c == LABEL_CHAR {
        true
    } else {
        false
    }
}

pub fn trim_comment(s: &mut String) {
    let index = s.find(COMMENT_CHAR);

    if let Some(i) = index {
        s.truncate(i);
    }
}

pub fn create_build_file(source_filename: &str) -> std::io::Result<()> {

    let source_code_file = OpenOptions::new()
        .write(false)
        .read(true)
        .open(source_filename)?;

    let build_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(BUILD_FILE_NAME)?;


    let src = BufReader::new(source_code_file);
    let mut build = BufWriter::new(build_file);

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




