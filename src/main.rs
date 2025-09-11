use std::fs::OpenOptions;
use std::io::Write;

mod converter;

const MEM_SIZE: usize = 256;
const FULL_SIZE: usize = 2*MEM_SIZE;
const CODE_SEG: usize = 0;
const DATA_SEG: usize = FULL_SIZE/2;

pub struct NeanderMem {
    arr: [u8; FULL_SIZE],
    code_seg: usize,
    data_seg: usize,
    code_seg_byte: usize,
    data_seg_byte: usize,
}

impl NeanderMem {

    fn new() -> Self {
        NeanderMem {
            arr: [0; FULL_SIZE],
            code_seg: CODE_SEG,
            data_seg: DATA_SEG,
            code_seg_byte: 0,
            data_seg_byte: 0,
        }
    }

    fn write_ins_addr(&mut self, token: Token, addr: Option<u8>) {
        let code_seg = self.code_seg;
        let mut curr_byte = self.code_seg_byte;
        let padding = 0x00;

        self.arr[code_seg + curr_byte] = token.to_opcode();
        curr_byte += 1;

        self.arr[code_seg + curr_byte] = padding;
        curr_byte += 1;

        if let Some(addr) = addr {
            self.arr[code_seg + curr_byte] = addr;
            curr_byte += 1;

            self.arr[code_seg + curr_byte] = padding;
            curr_byte += 1;
        }

        self.code_seg_byte = curr_byte;
    }

    fn to_output_file(&self, filename: &str) -> std::io::Result<()> {

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        file.write_all(&[0x03, 0x4e, 0x44, 0x52])?;
        file.write_all(&self.arr)?;

        Ok(())
    }
}

enum Token {
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

impl Token {

    fn from_str(s: &str) -> Result<Self, ()> {

        use Token::*;

        match s {
            "nop" => Ok(Nop),
            "sta" => Ok(Sta),
            "lda" => Ok(Lda),
            "add" => Ok(Add),
            "or" => Ok(Or),
            "and" => Ok(And),
            "not" => Ok(Not),
            "jmp" => Ok(Jmp),
            "jn" => Ok(Jn),
            "jz" => Ok(Jz),
            "hlt" => Ok(Hlt),
            _ => Err(()),
        }

    }

    fn to_opcode(&self) -> u8 {

        use Token::*;

        16 * match self {
            Nop => 0b0001,
            Sta => 0b0010,
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

const COMMENT_CHAR: char = ';';

fn trim_comment(s: &mut String) {
    let index = s.find(COMMENT_CHAR);

    if let Some(i) = index {
        s.truncate(i);
    }
}

fn main() -> std::io::Result<()> {

    let mut mem = NeanderMem::new();

    use Token::*;

    let mut minha_linha_entrada = String::from("lda;azul escuro");
    trim_comment(&mut minha_linha_entrada);
    let meu_token = Token::from_str(&minha_linha_entrada).unwrap();

    mem.write_ins_addr(meu_token, Some(128));
    mem.write_ins_addr(Add, Some(129));

    // Output file writing
    mem.to_output_file("output.mem")?;

    Ok(())
}
