use crate::{metadata::{memlayout::NeanderMem, Metadata}};

pub enum Mnemonic {
    Nop,
    Sta,
    Lda,
    Add,
    Or,
    And,
    Not,
    Jmp,
    Jn,
    Jz,
    Hlt,
}

impl Mnemonic {

    pub fn from_text(s: &str) -> Self {

        use Mnemonic::*;

        match s {
            "nop" => Nop,
            "sta" => Sta,
            "lda" => Lda,
            "add" => Add,
            "or" => Or,
            "and" => And,
            "not" => Not,
            "jmp" => Jmp,
            "jn" => Jn,
            "jz" => Jz,
            "hlt" => Hlt,
            _ => Nop
        }

    }

    pub fn to_opcode(&self) -> u8 {

        use Mnemonic::*;

        16 * match self {
            Nop => 0b0000,
            Sta => 0b0001,
            Lda => 0b0010,
            Add => 0b0011,
            Or => 0b0100,
            And => 0b0101,
            Not => 0b0110,
            Jmp => 0b1000,
            Jn => 0b1001,
            Jz => 0b1010,
            Hlt => 0b1111,
        }
    }
}

pub struct Instruction {

    pub mnemonic: Mnemonic,
    pub addr: Option<u8>,

}

impl Instruction {

    fn from_line(line: &str) -> Self {

        let mut tokens = line.split_whitespace();

        Instruction {
            mnemonic: Mnemonic::from_text(tokens.next().expect("NO EMPTY LINES EXPECTED")) ,
            addr: if let Some(addr ) = tokens.next() {
                addr.parse().ok()
            } else {
                None
            }
        }
    }
}

pub fn assemble(metadata: Metadata) -> NeanderMem {

    let mut mem = metadata.mem_layout;
    let immediates = metadata.label_info.immediates;

    let mut on_code;
    let mut on_data;

    if metadata.seg_info.code.addr == 0 {
        on_code = true;
        on_data = false;
    } else { // Assuming in this case that if the code addr is not zero
        // the text IS GOING TO start with data
        on_code = false;
        on_data = true;
    }

    let mut counting_words: usize = 0;

    for line in metadata.text.lines() {

        if on_code {
            if counting_words >= metadata.seg_info.code.len {
                on_code = false;
                on_data = true;
                counting_words = 0;
            }
        } else if counting_words >= metadata.seg_info.data.len {
                on_code = true;
                on_data = false;
                counting_words = 0;
        }

        counting_words += line.split_whitespace().count();

        if on_code {
            mem.write_instruction(Instruction::from_line(line));
        } else if on_data {
            mem.write_data(line.parse().expect("NO EMPTY LINES ALLOWED"));
        }
    }

    mem.write_reverse_data(immediates);

    mem

}






