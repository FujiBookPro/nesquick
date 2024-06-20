use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Opcode(pub Instruction, pub AddrMode, pub usize);

impl Opcode {
    pub fn decode(code: u8) -> Option<Self> {
        OPCODES.get(&code).map(|c| c.clone())
    }
}

#[derive(Clone)]
pub enum Instruction {
    ADC,
    LDA,
    STA,
}

#[derive(Clone, PartialEq)]
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
    IndexedIndirect,
    IndirectIndexed,
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
            Self::IndexedIndirect => 1,
            Self::IndirectIndexed => 1,
        }
    }
}

lazy_static! {
    static ref OPCODES: HashMap<u8, Opcode> = {
        let mut o = HashMap::new();

        o.insert(0x6d, Opcode(Instruction::ADC, AddrMode::Absolute, 4));

        o.insert(0xa9, Opcode(Instruction::LDA, AddrMode::Immediate, 2));

        o.insert(0x8d, Opcode(Instruction::STA, AddrMode::Absolute, 4));

        o
    };
}
