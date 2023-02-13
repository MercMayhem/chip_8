use std::{path::Path, fs::read};
use crate::memory::Memory;
use crate::opcode::{Opcode, OpcodeTypes};

struct Cpu{
    opcode : Opcode,
    memory : Memory
}

impl Cpu{
    fn initialize(&mut self, file_path: &str){
        let path = Path::new(file_path);
        let file = read(path).unwrap();

        if file.len() > 4096{
            panic!("Incorrect length of file")
        }
        for i in file.iter().enumerate(){
            self.memory.addr_mem[i.0 + 511] = *i.1
        }
    }

    fn fetch(&mut self, pc: u16){
        self.opcode.code = u16::from_be_bytes([self.memory.addr_mem[pc as usize], self.memory.addr_mem[(pc + 1) as usize]])
    }

    fn decode(&mut self){
        let kind = Opcode::find_kind(self.opcode.code).unwrap();
        self.opcode.kind = kind;
    }

    fn execute(&mut self){
        match self.opcode.kind{
            OpcodeTypes::CLS => todo!(),
            OpcodeTypes::RET => {
                self.memory.pc = self.memory.stack[self.memory.sp as usize].expect("None in stack");
                self.memory.sp -= 1;
            },
            OpcodeTypes::JPAddr => {
                self.memory.pc = self.opcode.code & 0x0FFF;
            },
            OpcodeTypes::CALLAddr => {
                self.memory.sp += 1;
                self.memory.stack[self.memory.sp as usize] = Some(self.memory.pc)
            },
            OpcodeTypes::SEVxByte => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg_no = bytes[0] & 0x0F;
                let comp_val = bytes[1];

                if self.memory.reg[reg_no as usize] == comp_val{
                    self.memory.pc += 2;
                }
            },
            OpcodeTypes::SNEVxByte => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg_no = bytes[0] & 0x0F;
                let comp_val = bytes[1];

                if self.memory.reg[reg_no as usize] != comp_val{
                    self.memory.pc += 2;
                }
            },
            OpcodeTypes::SEVxVy => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg1 = bytes[0] & 0x0F;
                let reg2 = bytes[1].rotate_left(4) & 0x0F;

                if self.memory.reg[reg1 as usize] == self.memory.reg[reg2 as usize]{
                    self.memory.pc += 2;
                }
            },
            OpcodeTypes::LDVxbyte => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg_no = bytes[0] & 0x0F;

                self.memory.reg[reg_no as usize] = bytes[1]
            },
            OpcodeTypes::ADDVxbyte => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg_no = bytes[0] & 0x0F;

                self.memory.reg[reg_no as usize] += bytes[1]
            },
        }
    }
}