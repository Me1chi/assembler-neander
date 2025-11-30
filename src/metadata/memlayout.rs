use std::{fs::OpenOptions, io::Write, vec};

use crate::{encoder::Instruction, metadata::labelinfo::immediatetrick::ImmediateAddressing};


pub const SIM_MEM_SIZE: usize = 256;

pub struct NeanderMem {
    pub arr: Vec<u8>,
    pub mem_size: usize,
    pub code_seg_byte: usize,
    pub data_seg_byte: usize,
    pub target_sim: bool,
}

impl Default for NeanderMem {
    fn default() -> Self {
        Self::new(true)
    }
}

impl NeanderMem {

    pub fn full_size(&self) -> usize {
        if self.target_sim {
            SIM_MEM_SIZE * 2
        } else {
            SIM_MEM_SIZE
        }
    }

    pub fn code_seg(&self) -> usize {
        0
    }

    pub fn data_seg(&self) -> usize {
        self.mem_size/2
    }

    pub fn new(target_sim: bool) -> Self {

        let vec_size = if target_sim {
            SIM_MEM_SIZE * 2
        } else {
            SIM_MEM_SIZE
        };

        NeanderMem {
            arr: vec![0; vec_size],
            mem_size: SIM_MEM_SIZE,
            code_seg_byte: 0,
            data_seg_byte: 0,
            target_sim,
        }
    }

    pub fn write_instruction(&mut self, ins: Instruction) {
        let code_seg = self.code_seg();
        let mut curr_byte = self.code_seg_byte;
        let padding = 0x00;

        self.arr[code_seg + curr_byte] = ins.mnemonic.to_opcode();

        if let Some(dst) = ins.destination {
            self.arr[code_seg + curr_byte] += dst << 2;
        }

        if let Some(src) = ins.source {
            self.arr[code_seg + curr_byte] += src;
        }

        curr_byte += 1;

        if self.target_sim {
            self.arr[code_seg + curr_byte] = padding;
            curr_byte += 1;
        }

        if let Some(addr) = ins.addr {
            self.arr[code_seg + curr_byte] = addr;
            curr_byte += 1;

            if self.target_sim {
                self.arr[code_seg + curr_byte] = padding;
                curr_byte += 1;
            }
        }

        self.code_seg_byte = curr_byte;
    }

    pub fn write_data(&mut self, data: u8) {
        let mut data_seg = self.data_seg(); // The *2 here is because how Weber implemented
        // the Simulator 
        if self.target_sim {
            data_seg *= 2;
        }
        let mut curr_byte = self.data_seg_byte;
        let padding = 0x00;

        self.arr[data_seg + curr_byte] = data;
        curr_byte += 1;

        //Padding
        if self.target_sim {
            self.arr[data_seg + curr_byte] = padding;
            curr_byte += 1;

        }

        self.data_seg_byte = curr_byte;
    }

    pub fn write_reverse_data(&mut self, data: Vec<ImmediateAddressing>) {

        let addr_multiplier = if self.target_sim {
            2
        } else {
            1
        };

        for i in data {
            self.arr[i.addr * addr_multiplier] = i.value;
        }
    }

    pub fn to_output_file(&self, filename: &str) -> std::io::Result<()> {

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        if self.target_sim {
            file.write_all(&[0x03, 0x4e, 0x44, 0x52])?;
        }

        file.write_all(&self.arr)?;

        Ok(())
    }
}

