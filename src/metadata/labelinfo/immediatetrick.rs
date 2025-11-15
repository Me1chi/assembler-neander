use crate::metadata::Metadata;


const IMMEDIATE_ADDR_CHAR: char = '$';

#[derive(Debug)]
pub struct ImmediateAddressing {
    pub value: u8,
    pub addr: usize,
}

impl ImmediateAddressing {

    pub fn resolve_immediates(mut input: Metadata) -> Result<Metadata, std::io::Error> {

        let mut dst = Vec::new();

        let last_mem_addr = input.mem_layout.mem_size - 1;
        let mut addrs: Vec<ImmediateAddressing> = Vec::new();

        let mut curr_addr = last_mem_addr;
        for line in input.text.lines() {
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

            dst.push(new_line);
        }

        input.text = dst.join("\n");
        input.label_info.immediates = addrs;

        Ok(input)

    }

}

