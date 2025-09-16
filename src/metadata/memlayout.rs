use std::{fs::OpenOptions, io::Write};
use crate::token::Token;


pub const MEM_SIZE: usize = 256;
const FULL_SIZE: usize = 2*MEM_SIZE;
const CODE_SEG: usize = 0;
const DATA_SEG: usize = MEM_SIZE/2;

pub struct NeanderMem {
    pub arr: [u8; FULL_SIZE],
    pub code_seg: usize,
    pub data_seg: usize,
    pub code_seg_byte: usize,
    pub data_seg_byte: usize,
}

impl Default for NeanderMem {
    fn default() -> Self {
        Self::new()
    }
}

impl NeanderMem {

    pub fn new() -> Self {
        NeanderMem {
            arr: [0; FULL_SIZE],
            code_seg: CODE_SEG,
            data_seg: DATA_SEG,
            code_seg_byte: 0,
            data_seg_byte: 0,
        }
    }

    pub fn write_ins_addr(&mut self, token: Token, addr: Option<u8>) {
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

    pub fn to_output_file(&self, filename: &str) -> std::io::Result<()> {

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

