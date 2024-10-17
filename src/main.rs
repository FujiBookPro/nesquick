mod bus;
mod cpu;
mod opcode;

use crate::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();

    let program = [0xa2, 0x17, 0xe8, 0xe8, 0x86, 0x05];

    cpu.load_program(&program[..], crate::bus::MemLocation(0x8000));
    cpu.run(0x8000);
}
