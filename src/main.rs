mod memory;
mod cpu;
mod opcode;
extern crate minifb;

use cpu::Cpu;
use minifb::Key;
use opcode::OpcodeTypes;

fn main() {
    let mut processor = Cpu::initialize(&r"C:\Users\amanr\OneDrive\Documents\Coding projects\rust\CHIP-8\chip_8\chip8-test-rom-master\c8_test.c8");

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
    }
}
