use std::{path::Path, fs::read};
use crate::memory::Memory;
use crate::opcode::{Opcode, OpcodeTypes};
extern crate rand;
use crate::cpu::rand::Rng;

struct Cpu{
    opcode : Opcode,
    memory : Memory,
    keyboard : [u8; 16]
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

    fn wait_input() -> u8 {
        todo!()
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
            OpcodeTypes::LDVxVy => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg1 = bytes[0] & 0x0F;
                let reg2 = bytes[1].rotate_left(4) & 0x0F;

                self.memory.reg[reg1 as usize] = self.memory.reg[reg2 as usize]
            },
            OpcodeTypes::ORVxVy =>{
                let bytes = self.opcode.code.to_be_bytes();
                let reg1 = bytes[0] & 0x0F;
                let reg2 = bytes[1].rotate_left(4) & 0x0F;

                self.memory.reg[reg1 as usize] = self.memory.reg[reg2 as usize] | self.memory.reg[reg1 as usize]
            },

            OpcodeTypes::ANDVxVy => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg1 = bytes[0] & 0x0F;
                let reg2 = bytes[1].rotate_left(4) & 0x0F;

                self.memory.reg[reg1 as usize] = self.memory.reg[reg2 as usize] & self.memory.reg[reg1 as usize]
            },
            OpcodeTypes::XORVxVy => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg1 = bytes[0] & 0x0F;
                let reg2 = bytes[1].rotate_left(4) & 0x0F;

                self.memory.reg[reg1 as usize] = self.memory.reg[reg2 as usize] ^ self.memory.reg[reg1 as usize]
            },
            OpcodeTypes::ADDVxVy => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg1 = bytes[0] & 0x0F;
                let reg2 = bytes[1].rotate_left(4) & 0x0F;

                self.memory.reg[15] = (self.memory.reg[reg1 as usize] as u16 + self.memory.reg[reg2 as usize] as u16 > 255) as u8;

                self.memory.reg[reg1 as usize] = ((self.memory.reg[reg1 as usize] as u16 + self.memory.reg[reg2 as usize] as u16) & 0x00FF).try_into().unwrap()
            },
            OpcodeTypes::SUBVxVy => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg1 = bytes[0] & 0x0F;
                let reg2 = bytes[1].rotate_left(4) & 0x0F;

                self.memory.reg[15] = (self.memory.reg[reg1 as usize] < self.memory.reg[reg2 as usize]) as u8;
                
                let bor = u16::from_be_bytes([self.memory.reg[15], self.memory.reg[reg1 as usize]]);
                self.memory.reg[reg1 as usize] = (bor - self.memory.reg[reg2 as usize] as u16) as u8
            },
            OpcodeTypes::SHRVxVy => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg = bytes[0] & 0x0F;

                let lsb = self.memory.reg[reg as usize] & 0b1;
                self.memory.reg[15] = lsb;

                self.memory.reg[reg as usize] >>= 1;
            },
            OpcodeTypes::SUBNVxVy => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg1 = bytes[0] & 0x0F;
                let reg2 = bytes[1].rotate_left(4) & 0x0F;

                self.memory.reg[15] = (self.memory.reg[reg1 as usize] > self.memory.reg[reg2 as usize]) as u8;

                let bor = u16::from_be_bytes([self.memory.reg[15], self.memory.reg[reg2 as usize]]);
                self.memory.reg[reg1 as usize] = (bor - self.memory.reg[reg1 as usize] as u16) as u8
            },
            OpcodeTypes::SHLVxVy => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg = bytes[0] & 0x0F;

                let msb = self.memory.reg[reg as usize] & 0b10000000;
                self.memory.reg[15] = msb;

                self.memory.reg[reg as usize] <<= 1;
            },
            OpcodeTypes::SNEVxVy => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg1 = bytes[0] & 0x0F;
                let reg2 = bytes[1].rotate_left(4) & 0x0F;

                if self.memory.reg[reg1 as usize] != self.memory.reg[reg2 as usize]{
                    self.memory.pc += 2;
                }
            },
            OpcodeTypes::LDIAddr => {
                let addr = self.opcode.code & 0xFFF;
                self.memory.i = addr
            },
            OpcodeTypes::JPV0Addr => {
                let addr = self.opcode.code & 0xFFF;
                self.memory.pc = self.memory.reg[0] as u16 + addr;
            },
            OpcodeTypes::RNDVxbyte => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg = bytes[0] & 0x0F;
                let byte = bytes[1];

                let random_byte: u8 = rand::thread_rng().gen();

                self.memory.reg[reg as usize] = random_byte & byte;
            },
            OpcodeTypes::DRWVxVyNibble => {
                todo!()
            },
            OpcodeTypes::SKPVx => {
                let bytes = self.opcode.code.to_be_bytes();
                let byte = bytes[0] & 0x0F;

                if self.keyboard[byte as usize] == 1{
                    self.memory.pc += 2
                }
            },
            OpcodeTypes::SKNPVx => {
                let bytes = self.opcode.code.to_be_bytes();
                let byte = bytes[0] & 0x0F;

                if self.keyboard[byte as usize] == 0{
                    self.memory.pc += 2
                }
            },
            OpcodeTypes::LDVxDT => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg = bytes[0] & 0x0F;

                self.memory.reg[reg as usize] = self.memory.delay;
            },
            OpcodeTypes::LDVxK => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg = bytes[0] & 0x0F;

                let k = Self::wait_input();
                self.memory.reg[reg as usize] = k;
            },
            OpcodeTypes::LDDTVx => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg = bytes[0] & 0x0F;

                self.memory.delay = self.memory.reg[reg as usize]
            },
            OpcodeTypes::LDSTVx => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg = bytes[0] & 0x0F;

                self.memory.sound = self.memory.reg[reg as usize]
            },
            OpcodeTypes::ADDIVx => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg = bytes[0] & 0x0F;

                self.memory.i += self.memory.reg[reg as usize] as u16;
            },
            OpcodeTypes::LDFVx => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg = bytes[0] & 0x0F;

                self.memory.i = (self.memory.reg[reg as usize] * 5) as u16
            },
            OpcodeTypes::LDBVx => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg = bytes[0] & 0x0F;

                let number = self.memory.reg[reg as usize];
                let hundred = number / 100;
                let tens = (number / 10) % 10;
                let ones = number % 10;

                self.memory.addr_mem[self.memory.i as usize] = hundred;
                self.memory.addr_mem[self.memory.i as usize + 1] = tens;
                self.memory.addr_mem[self.memory.i as usize + 2] = ones;
            },
            OpcodeTypes::LDIVx => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg = bytes[0] & 0x0F;

                for num in 0..=reg {
                    self.memory.addr_mem[(self.memory.i + num as u16) as usize] = self.memory.reg[reg as usize];
                }
            },
            OpcodeTypes::LDVxI => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg = bytes[0] & 0x0F;

                for num in 0..=reg {
                    self.memory.reg[reg as usize] = self.memory.addr_mem[(self.memory.i + num as u16) as usize];
                }
            }
        }
    }
}