use crate::metadata::Metadata;


const LABEL_CHAR: char = ':';

#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub addr: usize,
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

    pub fn resolve_label_defs(mut input: Metadata) -> Result<Metadata, std::io::Error> {

        let mut dst = Vec::new();
        let mut labels: Vec<Label> = Vec::new();
        let mut line_counter = 0;

        let seg_info = input.seg_info;
        let mem = input.mem_layout;

        for line in input.text.lines() {
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
                dst.push(to_write);
            }
        }

        input.text = dst.join("\n");
        input.label_info.labels = labels;
        input.mem_layout = mem;
        input.seg_info = seg_info;

        Ok(input)

    }
}

