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
    Brk,
    Clc,
    Cld,
    Cli,
    Clv,
    Dec,
    Dex,
    Dey,
    Lda,
    Ldx,
    Ldy,
    Sta,
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
}

lazy_static! {
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

        o.insert(0x00, Opcode(Instruction::Brk, AddrMode::Implicit, 7, CycleLenType::Constant));

        o.insert(0x18, Opcode(Instruction::Clc, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0xD8, Opcode(Instruction::Cld, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0x58, Opcode(Instruction::Cli, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0xB8, Opcode(Instruction::Clv, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0xC6, Opcode(Instruction::Dec, AddrMode::ZeroPage, 5, CycleLenType::Constant));
        o.insert(0xD6, Opcode(Instruction::Dec, AddrMode::ZeroPageX, 6, CycleLenType::Constant));
        o.insert(0xCE, Opcode(Instruction::Dec, AddrMode::Absolute, 6, CycleLenType::Constant));
        o.insert(0xDE, Opcode(Instruction::Dec, AddrMode::AbsoluteX, 7, CycleLenType::Constant));

        o.insert(0xCA, Opcode(Instruction::Dex, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0x88, Opcode(Instruction::Dey, AddrMode::Implicit, 2, CycleLenType::Constant));

        o.insert(0xA9, Opcode(Instruction::Lda, AddrMode::Immediate, 2, CycleLenType::Constant));
        o.insert(0xA5, Opcode(Instruction::Lda, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0xB5, Opcode(Instruction::Lda, AddrMode::ZeroPageX, 4, CycleLenType::Constant));
        o.insert(0xAD, Opcode(Instruction::Lda, AddrMode::Absolute, 4, CycleLenType::Constant));
        o.insert(0xBD, Opcode(Instruction::Lda, AddrMode::AbsoluteX, 4, CycleLenType::PageCrossed));
        o.insert(0xB9, Opcode(Instruction::Lda, AddrMode::AbsoluteY, 4, CycleLenType::PageCrossed));
        o.insert(0xA1, Opcode(Instruction::Lda, AddrMode::IndirectX, 6, CycleLenType::Constant));
        o.insert(0xB1, Opcode(Instruction::Lda, AddrMode::IndirectY, 5, CycleLenType::PageCrossed));

        o.insert(0xA2, Opcode(Instruction::Ldx, AddrMode::Immediate, 2, CycleLenType::Constant));

        o.insert(0xA0, Opcode(Instruction::Ldy, AddrMode::Immediate, 2, CycleLenType::Constant));

        o.insert(0x85, Opcode(Instruction::Sta, AddrMode::ZeroPage, 3, CycleLenType::Constant));
        o.insert(0x8d, Opcode(Instruction::Sta, AddrMode::Absolute, 4, CycleLenType::Constant));

        o
    };
}
