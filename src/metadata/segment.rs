use crate::metadata::{labelinfo::label::Label, Metadata};

#[derive(Debug)]
pub struct Segment {
    pub addr: usize,
    pub len: usize,
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
    pub code: Segment, // Will be the line 0 of the memory (or CODE_SEG)
    pub data: Segment, // Same but 0x80 (DATA_SEG)
}

impl Default for SegInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl SegInfo {
    pub fn new() -> Self {
        SegInfo {
            code: Segment::new(),
            data: Segment::new(),
        }
    }

    pub fn resolve_seginfo(mut input: Metadata) -> Result<Metadata, std::io::Error> {

        let mut info = SegInfo::new();
        let mut dst = Vec::new();

        let mut on_code_seg = false;
        let mut on_data_seg = false;

        let mut line_counter = 0;

        for line in input.text.lines() {
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

            let num = if Label::from_str_counter(line, line_counter).is_none() {
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
                dst.push(line.to_string());
            }
        }

        input.text = dst.join("\n");
        input.seg_info = info;

        Ok(input)

    }
}

