use std::{path::Path, fs::read};
use crate::memory::Memory;
use crate::opcode::Opcode;

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
            self.memory.addr_mem[i.0 + 512] = *i.1
        }
    }

    fn fetch(&mut self, pc: u16){
        self.opcode.code = u16::from_be_bytes([self.memory.addr_mem[pc as usize], self.memory.addr_mem[(pc + 1) as usize]])
    }

    fn decode(&mut self){
        let kind = Opcode::find_kind(self.opcode.code);
        self.opcode.kind = kind;
    }

    fn execute(&self){
        todo!()
    }
}