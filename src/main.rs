mod memory;
mod cpu;
mod opcode;
extern crate minifb;

use std::time::Instant;

use cpu::Cpu;
use minifb::Key;
use opcode::OpcodeTypes;

fn main() {
    let mut processor = Cpu::initialize(&r"C:\Users\amanr\OneDrive\Documents\Coding projects\rust\CHIP-8\chip_8\chip8-test-rom-master\Tetris [Fran Dachille, 1991].ch8");

    let mut last_cycle = Instant::now();

    while processor.window.is_open() && !processor.window.is_key_down(Key::Escape){
        processor.fetch();
        processor.decode();
        processor.execute();

        if processor.opcode.kind != Some(OpcodeTypes::CALLAddr) && processor.opcode.kind != Some(OpcodeTypes::SNEVxByte) && 
        processor.opcode.kind != Some(OpcodeTypes::RET) && processor.opcode.kind != Some(OpcodeTypes::JPAddr) && processor.opcode.kind != Some(OpcodeTypes::SEVxVy)
        && processor.opcode.kind != Some(OpcodeTypes::JPV0Addr) && processor.opcode.kind != Some(OpcodeTypes::SKPVx) && processor.opcode.kind != Some(OpcodeTypes::SKNPVx)
        && processor.opcode.kind != Some(OpcodeTypes::SEVxByte)
        {
            processor.memory.pc += 2;
        }

        if processor.opcode.kind == Some(OpcodeTypes::JPAddr){
            let addr = processor.opcode.code & 0xFFF;
            if processor.memory.addr_mem[addr as usize] == processor.opcode.code.to_be_bytes()[0] && processor.memory.addr_mem[(addr + 1) as usize] == processor.opcode.code.to_be_bytes()[1]{
                processor.reset();
            }
        }

        let now = Instant::now();
        let time_elapsed = now.duration_since(last_cycle);

        if time_elapsed.as_micros() >= 16670{
            if processor.memory.sound > 0{
                processor.memory.sound -= 1;
            }

            if processor.memory.delay > 0{
                processor.memory.delay -= 1;
            }

            last_cycle = now;
        }
    }
}
