use std::fs::File;
use std::{path::Path, fs::read};
use crate::memory::Memory;
use crate::opcode::{Opcode, OpcodeTypes};
extern crate rand;
use crate::cpu::rand::Rng;
extern crate minifb;
use minifb::{Window, WindowOptions, Key};
use std::collections::hash_map::HashMap;
use std::io::Write;

const FONT_SET: [[u8; 5]; 16] = [[0xF0,0x90,0x90,0x90,0xF0], [0x20,0x60,0x20,0x20,0x70,], [0xF0,0x10,0xF0,0x80,0xF0], [0xF0,0x10,0xF0,0x10,0xF0], [0x90,0x90,0xF0,0x10,0x10], 
[0xF0,0x80,0xF0,0x10,0xF0], [0xF0,0x80,0xF0,0x90,0xF0], [0xF0,0x10,0x20,0x40,0x40], [0xF0,0x90,0xF0,0x90,0xF0], [0xF0,0x90,0xF0,0x10,0xF0], 
[0xF0,0x90,0xF0,0x90,0x90], [0xE0,0x90,0xE0,0x90,0xE0], [0xF0,0x80,0x80,0x80,0xF0], [0xE0,0x90,0x90,0x90,0xE0], [0xF0,0x80,0xF0,0x80,0xF0], [0xF0,0x80,0xF0,0x80,0x80]];

pub struct Cpu{
    pub opcode : Opcode,
    pub memory : Memory,
    pub window : Window,
    pub key_map : HashMap<Key, u8>,
    pub curr_buffer : [[u32;64];32]
}

impl Cpu{
    pub fn initialize(file_path: &str) -> Cpu{
        let path = Path::new(file_path);
        let file_vec = read(path).unwrap();

        let mut file = File::create("output.txt").expect("Failed to create file");

        for item in file_vec.iter() {
            writeln!(&mut file, "{:X?}", item).expect("Failed to write to file");
        }
            

        let mut addr_mem: [u8; 4096] = [0; 4096];

        if file_vec.len() > 4096{
            panic!("Incorrect length of file")
        }

        for (start, bytes) in FONT_SET.iter().enumerate(){
            addr_mem[(5 * start)] = bytes[0];
            addr_mem[(5 * start) + 1] = bytes[1];
            addr_mem[(5 * start) + 2] = bytes[2];
            addr_mem[(5 * start) + 3] = bytes[3];
            addr_mem[(5 * start) + 4] = bytes[4];
        }

        for i in file_vec.iter().enumerate(){
            addr_mem[i.0 + 512] = *i.1
        }

        let memory = Memory{
            addr_mem,
            reg : [0; 16],
            i : 0,
            pc : 0x200,
            stack : [None; 16],
            sp : 0,
            delay : 0,
            sound : 0
        };

        let opcode = Opcode{
            code : 0,
            kind : None
        };

        let mut window = Window::new("CHIP-8", 640, 320, WindowOptions::default()).unwrap();
        window.update_with_buffer(&[0; 2048], 64, 32).unwrap();

        let curr_buffer = [[0;64];32];

        let key_map = HashMap::from([
            (Key::Key1,1 as u8),
            (Key::Key2,2),
            (Key::Key3,3),
            (Key::Key4,12),
            (Key::Q,4),
            (Key::W,5),
            (Key::E,6),
            (Key::R,13),
            (Key::A,7),
            (Key::S,8),
            (Key::D,9),
            (Key::F,14),
            (Key::Z,10),
            (Key::X,0),
            (Key::C,11),
            (Key::V,15),
        ]);

        let mut file2 = File::create("output2.txt").expect("Failed to create file");
        for i in addr_mem.iter().enumerate(){
            writeln!(&mut file2, "{} : {:X?}", i.0, i.1).expect("Failed to write to file");
        }

        Cpu {opcode, memory, window, key_map, curr_buffer}
    }

    pub fn fetch(&mut self){
        self.opcode.code = u16::from_be_bytes([self.memory.addr_mem[self.memory.pc as usize], self.memory.addr_mem[(self.memory.pc + 1) as usize]]);
        println!("{:X?}", self.opcode.code)
    }

    pub fn decode(&mut self){
        let kind = Opcode::find_kind(self.opcode.code).unwrap();
        println!("{:?}\n", kind);
        self.opcode.kind = Some(kind);
    }

    pub fn execute(&mut self){
        match self.opcode.kind.as_ref().expect("incorrect opcode"){
            OpcodeTypes::CLS => {
                self.window.update_with_buffer(&[0; 64 * 32], 64, 32).unwrap()
            },
            OpcodeTypes::RET => {
                self.memory.sp -= 1;
                self.memory.pc = self.memory.stack[self.memory.sp as usize].expect("None in stack") + 2;
                self.memory.stack[self.memory.sp as usize] = None;
            },
            OpcodeTypes::JPAddr => {
                self.memory.pc = self.opcode.code & 0x0FFF;
            },
            OpcodeTypes::CALLAddr => {
                self.memory.stack[self.memory.sp as usize] = Some(self.memory.pc);
                self.memory.sp += 1;
                self.memory.pc = self.opcode.code & 0xFFF;
            },
            OpcodeTypes::SEVxByte => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg_no = bytes[0] & 0x0F;
                let comp_val = bytes[1];

                if self.memory.reg[reg_no as usize] == comp_val{
                    self.memory.pc += 4;
                }
                else {
                    self.memory.pc += 2;
                }
            },
            OpcodeTypes::SNEVxByte => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg_no = bytes[0] & 0x0F;
                let comp_val = bytes[1];

                if self.memory.reg[reg_no as usize] != comp_val{
                    self.memory.pc += 4;
                }
                else{
                    self.memory.pc += 2;
                }
            },
            OpcodeTypes::SEVxVy => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg1 = bytes[0] & 0x0F;
                let reg2 = bytes[1].rotate_left(4) & 0x0F;

                if self.memory.reg[reg1 as usize] == self.memory.reg[reg2 as usize]{
                    self.memory.pc += 4;
                }
                else{
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

                self.memory.reg[reg_no as usize] = self.memory.reg[reg_no as usize].wrapping_add(bytes[1])
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
                let bytes = self.opcode.code.to_be_bytes();
                let reg_x = bytes[0] & 0x0F;
                let reg_y = bytes[1].rotate_left(4) & 0x0F;
                let n = bytes[1] & 0x0F;
                let x_coord = self.memory.reg[reg_x as usize];
                let y_coord = self.memory.reg[reg_y as usize];
                
                self.memory.reg[15] = 0;

                for row in 0..n{
                    let byte = self.memory.addr_mem[(self.memory.i + row as u16) as usize];
                    let rel_y_coord = y_coord + row;

                    for pixel in 0..8_u8{
                        let bit = byte & (0x1 << 7 - pixel);
                        let rel_x_coord = x_coord + pixel;
                        
                        if bit != 0{
                            if self.curr_buffer[rel_y_coord as usize][rel_x_coord as usize] == 0{
                                self.curr_buffer[rel_y_coord as usize][rel_x_coord as usize] = 0xFFFFFF
                            }

                            else{
                                self.curr_buffer[rel_y_coord as usize][rel_x_coord as usize] = 0;
                                self.memory.reg[15] = 1;
                            }
                        }
                        if rel_x_coord == 63{
                            break;
                        }
                    }
                    if rel_y_coord == 31{
                        break;
                    }
                }

                let mut flattened_buffer = [0_u32; 2048];
                for row in 0..32_u16{
                    let start_index = row * 64;
                    let end_index = start_index + 64;

                    flattened_buffer[start_index as usize..end_index as usize].copy_from_slice(&self.curr_buffer[row as usize][..]);
                }

                self.window.update_with_buffer(&flattened_buffer, 64, 32).unwrap();
            },
            OpcodeTypes::SKPVx => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg = bytes[0] & 0x0F;

                let key_as_chip8 = self.memory.reg[reg as usize];
                let real_key = self.key_map.iter().find_map(|(key, &val)| if val == key_as_chip8 {Some(key)} else {None});

                if self.window.is_key_down(*real_key.expect("Vx did not contain a Key value")){
                    self.memory.pc += 4;
                }
                else {
                    self.memory.pc += 2;
                }
            },
            OpcodeTypes::SKNPVx => {
                let bytes = self.opcode.code.to_be_bytes();
                let reg = bytes[0] & 0x0F;

                let key_as_chip8 = self.memory.reg[reg as usize];
                let real_key = self.key_map.iter().find_map(|(key, &val)| if val == key_as_chip8 {Some(key)} else {None});

                if !self.window.is_key_down(*real_key.expect("Vx did not contain a Key value")){
                    self.memory.pc += 4
                }

                else {
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

                let k = self.window.get_keys();
                self.memory.reg[reg as usize] = self.key_map[&k[0]];
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

    pub fn reset(&mut self){
        self.memory.pc = 512;
        self.memory.reg = [0; 16];
        self.memory.i = 0;
        self.memory.stack = [None; 16];
        self.memory.sp = 0;
        self.memory.delay = 0;
        self.memory.sound = 0;

        self.curr_buffer = [[0;64];32];
        self.window.update_with_buffer(&[0; 2048], 64, 32).unwrap();
    }
}