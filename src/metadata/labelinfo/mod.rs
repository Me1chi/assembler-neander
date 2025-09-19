use crate::metadata::{labelinfo::{immediatetrick::ImmediateAddressing, label::Label}, Metadata};

pub mod immediatetrick;
pub mod label;

#[derive(Debug)]
pub struct LabelInfo {
    pub labels: Vec<Label>,
    pub immediates: Vec<ImmediateAddressing>,
}

impl Default for LabelInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl LabelInfo {

    pub fn new() -> Self {
        LabelInfo {
            labels: Vec::new(),
            immediates: Vec::new(),
        }
    }

    pub fn apply_labels(mut input: Metadata) -> Result<Metadata, std::io::Error> {

        let mut dst = Vec::new();

        for line in input.text.lines() {
            let maybe_label = line
                .split_whitespace()
                .last()
                .expect("THERE SHOULD BE NO EMPTY LINES")
                .to_string();

            let mut new_line_vec: Vec<&str> = line
                .split_whitespace()
                .collect();

            let true_addr: String;
            if let Some(label) = input.label_info.labels
                .iter()
                .find(|x| x.name == maybe_label) {


                true_addr = label.addr.to_string();
                new_line_vec.pop();
                new_line_vec.push(&true_addr);
            }

            let line = new_line_vec.join(" ");
            dst.push(line);
        }

        input.text = dst.join("\n");

        Ok(input)

    }

}

