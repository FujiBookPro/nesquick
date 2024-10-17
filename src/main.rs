mod bus;
mod cpu;
mod opcode;

use crate::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();

    #[rustfmt::skip]
    let program = [0xa9, 0x02, 0x85, 0x01, 0xa9, 0x81, 0x85, 0x03, 0xa9, 0x00, 0xa2, 0x08, 0x46, 0x03, 0x90, 0x03, 0x18, 0x65, 0x01, 0x6a, 0x66, 0x05, 0xca, 0xd0, 0xf3, 0x85, 0x06];

    cpu.load_program(&program[..], crate::bus::MemLocation(0x8000));
    cpu.run(0x8000);
}
