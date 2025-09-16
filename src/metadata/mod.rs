use crate::{metadata::{labelinfo::LabelInfo, memlayout::NeanderMem, segment::SegInfo}, utils::strutils::trim_comment};

pub mod memlayout;
pub mod segment;
pub mod labelinfo;

pub struct Metadata {
    pub seg_info: SegInfo,
    pub label_info: LabelInfo,
    pub mem_layout: NeanderMem,
    pub text: String,
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}

impl Metadata {

    pub fn new() -> Self {
        Metadata {
            seg_info: SegInfo::new(),
            label_info: LabelInfo::new(),
            mem_layout: NeanderMem::new(),
            text: String::new(),
        }
    }

}

pub fn to_lower_chop_comment(mut input: Metadata) -> Result<Metadata, std::io::Error> {

    let mut dst = Vec::new();

    for line in input.text.lines() {

        let mut line = line.to_lowercase();

        trim_comment(&mut line);
        let line = line.trim();

        if !line.is_empty() {
            dst.push(line.to_string());
        }
    }

    input.text = dst.join("\n");

    Ok(input)
}

