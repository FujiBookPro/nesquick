mod cpu;
mod opcode;

use crate::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();

    // LDA #$69
    // STA $40

    // LDA #$40
    // STA $03

    // LDX #$01

    // LDA (02, X)
    let program = [0xa9, 0x80, 0x85, 0x01, 0x65, 0x01];

    cpu.load_program(&program[..], 0x0600);
    cpu.run(0x0600);
}
