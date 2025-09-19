use std::{fs::OpenOptions, io::Write};

use crate::{encoder::Instruction, metadata::labelinfo::immediatetrick::ImmediateAddressing};


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

    pub fn write_instruction(&mut self, ins: Instruction) {
        let code_seg = self.code_seg;
        let mut curr_byte = self.code_seg_byte;
        let padding = 0x00;

        self.arr[code_seg + curr_byte] = ins.mnemonic.to_opcode();
        curr_byte += 1;

        self.arr[code_seg + curr_byte] = padding;
        curr_byte += 1;

        if let Some(addr) = ins.addr {
            self.arr[code_seg + curr_byte] = addr;
            curr_byte += 1;

            self.arr[code_seg + curr_byte] = padding;
            curr_byte += 1;
        }

        self.code_seg_byte = curr_byte;
    }

    pub fn write_data(&mut self, data: u8) {
        let data_seg = self.data_seg * 2; // The *2 here is because how Weber implemented
        // the Simulator 
        let mut curr_byte = self.data_seg_byte;
        let padding = 0x00;

        self.arr[data_seg + curr_byte] = data;
        curr_byte += 1;

        //Padding
        self.arr[data_seg + curr_byte] = padding;
        curr_byte += 1;

        self.data_seg_byte = curr_byte;
    }

    pub fn write_reverse_data(&mut self, data: Vec<ImmediateAddressing>) {
        for i in data {
            self.arr[i.addr * 2] = i.value;
        }
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

