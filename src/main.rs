mod cpu;
mod opcode;

use crate::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();

    // LDA #$6
    // STA $0200
    // LDA #$13
    // ADC $0200
    // STA $0201
    let program = vec![0xa9, 0x06, 0x8d, 0x00, 0x02, 0xa9, 0x0d, 0x6d, 0x00, 0x02, 0x8d, 0x01, 0x02];

    cpu.load_program(&program[..]);
    cpu.run();
}
