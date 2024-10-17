use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Opcode(pub Instruction, pub AddrMode, pub usize, pub CycleLenType);

impl Opcode {
    pub fn decode(code: u8) -> Option<Self> {
        OPCODES.get(&code).cloned()
    }
}

#[derive(Clone, PartialEq)]
pub enum Instruction {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rts,
    Sta,
    Stx,
    Sty,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AddrMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

impl AddrMode {
    pub fn n_param_bytes(&self) -> usize {
        match self {
            Self::Implicit => 0,
            Self::Accumulator => 0,
            Self::Immediate => 1,
            Self::ZeroPage => 1,
            Self::ZeroPageX => 1,
            Self::ZeroPageY => 1,
            Self::Relative => 1,
            Self::Absolute => 2,
            Self::AbsoluteX => 2,
            Self::AbsoluteY => 2,
            Self::Indirect => 2,
            Self::IndirectX => 1,
            Self::IndirectY => 1,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum CycleLenType {
    Constant,
    PageCrossed,
    Branch,
}

lazy_static! {
    #[rustfmt::skip]
    static ref OPCODES: HashMap<u8, Opcode> = {
        let mut o = HashMap::new();

        o.insert(0x69, Opcode(Instruction::Adc, AddrMode::Immediate, 2, CycleLenType::Constant));
        o.insert(0x65, Opcode(Instruction::Adc, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0x75, Opcode(Instruction::Adc, AddrMode::ZeroPageX, 4, CycleLenType::Constant));
        o.insert(0x6d, Opcode(Instruction::Adc, AddrMode::Absolute, 4, CycleLenType::Constant));
        o.insert(0x7d, Opcode(Instruction::Adc, AddrMode::AbsoluteX, 4, CycleLenType::PageCrossed));
        o.insert(0x79, Opcode(Instruction::Adc, AddrMode::AbsoluteY, 4, CycleLenType::PageCrossed));
        o.insert(0x61, Opcode(Instruction::Adc, AddrMode::IndirectX, 6, CycleLenType::Constant));
        o.insert(0x71, Opcode(Instruction::Adc, AddrMode::IndirectY, 5, CycleLenType::PageCrossed));

        o.insert(0x29, Opcode(Instruction::And, AddrMode::Immediate, 2, CycleLenType::Constant));
        o.insert(0x25, Opcode(Instruction::And, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0x35, Opcode(Instruction::And, AddrMode::ZeroPageX, 4, CycleLenType::Constant));
        o.insert(0x2D, Opcode(Instruction::And, AddrMode::Absolute, 4, CycleLenType::Constant));
        o.insert(0x3D, Opcode(Instruction::And, AddrMode::AbsoluteX, 4, CycleLenType::PageCrossed));
        o.insert(0x39, Opcode(Instruction::And, AddrMode::AbsoluteY, 4, CycleLenType::PageCrossed));
        o.insert(0x21, Opcode(Instruction::And, AddrMode::IndirectX, 6, CycleLenType::Constant));
        o.insert(0x31, Opcode(Instruction::And, AddrMode::IndirectY, 5, CycleLenType::PageCrossed));

        o.insert(0x0A, Opcode(Instruction::Asl, AddrMode::Accumulator, 2, CycleLenType::Constant));
        o.insert(0x06, Opcode(Instruction::Asl, AddrMode::ZeroPage, 5, CycleLenType::Constant));
        o.insert(0x16, Opcode(Instruction::Asl, AddrMode::ZeroPageX, 6, CycleLenType::Constant));
        o.insert(0x0E, Opcode(Instruction::Asl, AddrMode::Absolute, 6, CycleLenType::Constant));
        o.insert(0x1E, Opcode(Instruction::Asl, AddrMode::AbsoluteX, 7, CycleLenType::Constant));

        o.insert(0x90, Opcode(Instruction::Bcc, AddrMode::Relative, 2, CycleLenType::Branch));

        o.insert(0xB0, Opcode(Instruction::Bcs, AddrMode::Relative, 2, CycleLenType::Branch));

        o.insert(0xF0, Opcode(Instruction::Beq, AddrMode::Relative, 2, CycleLenType::Branch));

        o.insert(0x30, Opcode(Instruction::Bmi, AddrMode::Relative, 2, CycleLenType::Branch));

        o.insert(0xD0, Opcode(Instruction::Bne, AddrMode::Relative, 2, CycleLenType::Branch));

        o.insert(0x10, Opcode(Instruction::Bpl, AddrMode::Relative, 2, CycleLenType::Branch));

        o.insert(0x00, Opcode(Instruction::Brk, AddrMode::Implicit, 7, CycleLenType::Constant));

        o.insert(0x50, Opcode(Instruction::Bvc, AddrMode::Relative, 2, CycleLenType::Branch));

        o.insert(0x70, Opcode(Instruction::Bvs, AddrMode::Relative, 2, CycleLenType::Branch));

        o.insert(0x18, Opcode(Instruction::Clc, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0xD8, Opcode(Instruction::Cld, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0x58, Opcode(Instruction::Cli, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0xB8, Opcode(Instruction::Clv, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0xC9, Opcode(Instruction::Cmp, AddrMode::Immediate, 2, CycleLenType::Constant));
        o.insert(0xC5, Opcode(Instruction::Cmp, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0xD5, Opcode(Instruction::Cmp, AddrMode::ZeroPageX, 4, CycleLenType::Constant));
        o.insert(0xCD, Opcode(Instruction::Cmp, AddrMode::Absolute, 4, CycleLenType::Constant));
        o.insert(0xDD, Opcode(Instruction::Cmp, AddrMode::AbsoluteX, 4, CycleLenType::PageCrossed));
        o.insert(0xD9, Opcode(Instruction::Cmp, AddrMode::AbsoluteY, 4, CycleLenType::PageCrossed));
        o.insert(0xC1, Opcode(Instruction::Cmp, AddrMode::IndirectX, 6, CycleLenType::Constant));
        o.insert(0xD1, Opcode(Instruction::Cmp, AddrMode::IndirectY, 5, CycleLenType::PageCrossed));

        o.insert(0xE0, Opcode(Instruction::Cpx, AddrMode::Immediate, 2, CycleLenType::Constant));
        o.insert(0xE4, Opcode(Instruction::Cpx, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0xEC, Opcode(Instruction::Cpx, AddrMode::Absolute, 4, CycleLenType::Constant));

        o.insert(0xC0, Opcode(Instruction::Cpy, AddrMode::Immediate, 2, CycleLenType::Constant));
        o.insert(0xC4, Opcode(Instruction::Cpy, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0xCC, Opcode(Instruction::Cpy, AddrMode::Absolute, 4, CycleLenType::Constant));

        o.insert(0xC6, Opcode(Instruction::Dec, AddrMode::ZeroPage, 5, CycleLenType::Constant));
        o.insert(0xD6, Opcode(Instruction::Dec, AddrMode::ZeroPageX, 6, CycleLenType::Constant));
        o.insert(0xCE, Opcode(Instruction::Dec, AddrMode::Absolute, 6, CycleLenType::Constant));
        o.insert(0xDE, Opcode(Instruction::Dec, AddrMode::AbsoluteX, 7, CycleLenType::Constant));

        o.insert(0xCA, Opcode(Instruction::Dex, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0x88, Opcode(Instruction::Dey, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0x49, Opcode(Instruction::Eor, AddrMode::Immediate, 2, CycleLenType::Constant));
        o.insert(0x45, Opcode(Instruction::Eor, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0x55, Opcode(Instruction::Eor, AddrMode::ZeroPageX, 4, CycleLenType::Constant));
        o.insert(0x4D, Opcode(Instruction::Eor, AddrMode::Absolute, 4, CycleLenType::Constant));
        o.insert(0x5D, Opcode(Instruction::Eor, AddrMode::AbsoluteX, 4, CycleLenType::PageCrossed));
        o.insert(0x59, Opcode(Instruction::Eor, AddrMode::AbsoluteY, 4, CycleLenType::PageCrossed));
        o.insert(0x41, Opcode(Instruction::Eor, AddrMode::IndirectX, 6, CycleLenType::Constant));
        o.insert(0x51, Opcode(Instruction::Eor, AddrMode::IndirectY, 5, CycleLenType::PageCrossed));

        o.insert(0xE6, Opcode(Instruction::Inc, AddrMode::ZeroPage, 5, CycleLenType::Constant));
        o.insert(0xF6, Opcode(Instruction::Inc, AddrMode::ZeroPageX, 6, CycleLenType::Constant));
        o.insert(0xEE, Opcode(Instruction::Inc, AddrMode::ZeroPage, 6, CycleLenType::Constant));
        o.insert(0xFE, Opcode(Instruction::Inc, AddrMode::ZeroPage, 7, CycleLenType::Constant));

        o.insert(0xE8, Opcode(Instruction::Inx, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0xC8, Opcode(Instruction::Iny, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0x4C, Opcode(Instruction::Jmp, AddrMode::Absolute, 3, CycleLenType::Constant));
        o.insert(0x6C, Opcode(Instruction::Jmp, AddrMode::Indirect, 5, CycleLenType::Constant));

        o.insert(0x20, Opcode(Instruction::Jsr, AddrMode::Absolute, 6, CycleLenType::Constant));

        o.insert(0xA9, Opcode(Instruction::Lda, AddrMode::Immediate, 2, CycleLenType::Constant));
        o.insert(0xA5, Opcode(Instruction::Lda, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0xB5, Opcode(Instruction::Lda, AddrMode::ZeroPageX, 4, CycleLenType::Constant));
        o.insert(0xAD, Opcode(Instruction::Lda, AddrMode::Absolute, 4, CycleLenType::Constant));
        o.insert(0xBD, Opcode(Instruction::Lda, AddrMode::AbsoluteX, 4, CycleLenType::PageCrossed));
        o.insert(0xB9, Opcode(Instruction::Lda, AddrMode::AbsoluteY, 4, CycleLenType::PageCrossed));
        o.insert(0xA1, Opcode(Instruction::Lda, AddrMode::IndirectX, 6, CycleLenType::Constant));
        o.insert(0xB1, Opcode(Instruction::Lda, AddrMode::IndirectY, 5, CycleLenType::PageCrossed));

        o.insert(0xA2, Opcode(Instruction::Ldx, AddrMode::Immediate, 2, CycleLenType::Constant));
        o.insert(0xA6, Opcode(Instruction::Ldx, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0xB6, Opcode(Instruction::Ldx, AddrMode::ZeroPageY, 4, CycleLenType::Constant));
        o.insert(0xAE, Opcode(Instruction::Ldx, AddrMode::Absolute, 4, CycleLenType::Constant));
        o.insert(0xBE, Opcode(Instruction::Ldx, AddrMode::AbsoluteY, 4, CycleLenType::PageCrossed));

        o.insert(0xA0, Opcode(Instruction::Ldy, AddrMode::Immediate, 2, CycleLenType::Constant));
        o.insert(0xA4, Opcode(Instruction::Ldy, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0xB4, Opcode(Instruction::Ldy, AddrMode::ZeroPageX, 4, CycleLenType::Constant));
        o.insert(0xAC, Opcode(Instruction::Ldy, AddrMode::Absolute, 4, CycleLenType::Constant));
        o.insert(0xBC, Opcode(Instruction::Ldy, AddrMode::AbsoluteX, 4, CycleLenType::PageCrossed));

        o.insert(0x4A, Opcode(Instruction::Lsr, AddrMode::Accumulator, 2, CycleLenType::Constant));
        o.insert(0x46, Opcode(Instruction::Lsr, AddrMode::ZeroPage, 5, CycleLenType::Constant));
        o.insert(0x56, Opcode(Instruction::Lsr, AddrMode::ZeroPageX, 6, CycleLenType::Constant));
        o.insert(0x4E, Opcode(Instruction::Lsr, AddrMode::Absolute, 6, CycleLenType::Constant));
        o.insert(0x5E, Opcode(Instruction::Lsr, AddrMode::AbsoluteX, 7, CycleLenType::Constant));

        o.insert(0xEA, Opcode(Instruction::Nop, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0x09, Opcode(Instruction::Ora, AddrMode::Immediate, 2, CycleLenType::Constant));
        o.insert(0x05, Opcode(Instruction::Ora, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0x15, Opcode(Instruction::Ora, AddrMode::ZeroPageX, 4, CycleLenType::Constant));
        o.insert(0x0D, Opcode(Instruction::Ora, AddrMode::Absolute, 4, CycleLenType::Constant));
        o.insert(0x1D, Opcode(Instruction::Ora, AddrMode::AbsoluteX, 4, CycleLenType::Constant));
        o.insert(0x19, Opcode(Instruction::Ora, AddrMode::AbsoluteY, 4, CycleLenType::PageCrossed));
        o.insert(0x01, Opcode(Instruction::Ora, AddrMode::IndirectX, 6, CycleLenType::Constant));
        o.insert(0x11, Opcode(Instruction::Ora, AddrMode::IndirectY, 5, CycleLenType::PageCrossed));

        o.insert(0x48, Opcode(Instruction::Pha, AddrMode::Implicit, 3, CycleLenType::Constant));

        o.insert(0x08, Opcode(Instruction::Php, AddrMode::Implicit, 3, CycleLenType::Constant));

        o.insert(0x68, Opcode(Instruction::Pla, AddrMode::Implicit, 3, CycleLenType::Constant));

        o.insert(0x28, Opcode(Instruction::Plp, AddrMode::Implicit, 3, CycleLenType::Constant));

        o.insert(0x2A, Opcode(Instruction::Rol, AddrMode::Accumulator, 2, CycleLenType::Constant));
        o.insert(0x26, Opcode(Instruction::Rol, AddrMode::ZeroPage, 5, CycleLenType::Constant));
        o.insert(0x36, Opcode(Instruction::Rol, AddrMode::ZeroPageX, 6, CycleLenType::Constant));
        o.insert(0x2E, Opcode(Instruction::Rol, AddrMode::Absolute, 6, CycleLenType::Constant));
        o.insert(0x3E, Opcode(Instruction::Rol, AddrMode::AbsoluteX, 7, CycleLenType::Constant));

        o.insert(0x6A, Opcode(Instruction::Ror, AddrMode::Accumulator, 2, CycleLenType::Constant));
        o.insert(0x66, Opcode(Instruction::Ror, AddrMode::ZeroPage, 5, CycleLenType::Constant));
        o.insert(0x76, Opcode(Instruction::Ror, AddrMode::ZeroPageX, 6, CycleLenType::Constant));
        o.insert(0x6E, Opcode(Instruction::Ror, AddrMode::Absolute, 6, CycleLenType::Constant));
        o.insert(0x7E, Opcode(Instruction::Ror, AddrMode::AbsoluteX, 7, CycleLenType::Constant));

        o.insert(0x60, Opcode(Instruction::Rts, AddrMode::Implicit, 6, CycleLenType::Constant));

        o.insert(0x85, Opcode(Instruction::Sta, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0x8d, Opcode(Instruction::Sta, AddrMode::Absolute, 4, CycleLenType::Constant));

        o.insert(0x86, Opcode(Instruction::Stx, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0x96, Opcode(Instruction::Stx, AddrMode::ZeroPageY, 4, CycleLenType::Constant));
        o.insert(0x8E, Opcode(Instruction::Stx, AddrMode::Absolute, 4, CycleLenType::Constant));

        o.insert(0x84, Opcode(Instruction::Sty, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0x94, Opcode(Instruction::Sty, AddrMode::ZeroPageX, 4, CycleLenType::Constant));
        o.insert(0x8C, Opcode(Instruction::Sty, AddrMode::Absolute, 4, CycleLenType::Constant));

        o
    };
}
