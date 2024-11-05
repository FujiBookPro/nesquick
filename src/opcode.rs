use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Opcode(pub Instruction, pub AddrMode, pub CycleLen);

impl Opcode {
    pub fn decode(code: u8) -> Option<Self> {
        OPCODES.get(&code).cloned()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Instruction {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
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
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
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

#[derive(Clone)]
pub enum CycleLen {
    Constant(usize),
    PageCrossed(usize),
    Branch,
}

lazy_static! {
    #[rustfmt::skip]
    static ref OPCODES: HashMap<u8, Opcode> = {
        let mut o = HashMap::new();

        o.insert(0x69, Opcode(Instruction::Adc, AddrMode::Immediate, CycleLen::Constant(2)));
        o.insert(0x65, Opcode(Instruction::Adc, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0x75, Opcode(Instruction::Adc, AddrMode::ZeroPageX, CycleLen::Constant(4)));
        o.insert(0x6d, Opcode(Instruction::Adc, AddrMode::Absolute, CycleLen::Constant(4)));
        o.insert(0x7d, Opcode(Instruction::Adc, AddrMode::AbsoluteX, CycleLen::PageCrossed(4)));
        o.insert(0x79, Opcode(Instruction::Adc, AddrMode::AbsoluteY, CycleLen::PageCrossed(4)));
        o.insert(0x61, Opcode(Instruction::Adc, AddrMode::IndirectX, CycleLen::Constant(6)));
        o.insert(0x71, Opcode(Instruction::Adc, AddrMode::IndirectY, CycleLen::PageCrossed(5)));

        o.insert(0x29, Opcode(Instruction::And, AddrMode::Immediate, CycleLen::Constant(2)));
        o.insert(0x25, Opcode(Instruction::And, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0x35, Opcode(Instruction::And, AddrMode::ZeroPageX, CycleLen::Constant(4)));
        o.insert(0x2D, Opcode(Instruction::And, AddrMode::Absolute, CycleLen::Constant(4)));
        o.insert(0x3D, Opcode(Instruction::And, AddrMode::AbsoluteX, CycleLen::PageCrossed(4)));
        o.insert(0x39, Opcode(Instruction::And, AddrMode::AbsoluteY, CycleLen::PageCrossed(4)));
        o.insert(0x21, Opcode(Instruction::And, AddrMode::IndirectX, CycleLen::Constant(6)));
        o.insert(0x31, Opcode(Instruction::And, AddrMode::IndirectY, CycleLen::PageCrossed(5)));

        o.insert(0x0A, Opcode(Instruction::Asl, AddrMode::Accumulator, CycleLen::Constant(2)));
        o.insert(0x06, Opcode(Instruction::Asl, AddrMode::ZeroPage, CycleLen::Constant(5)));
        o.insert(0x16, Opcode(Instruction::Asl, AddrMode::ZeroPageX, CycleLen::Constant(6)));
        o.insert(0x0E, Opcode(Instruction::Asl, AddrMode::Absolute, CycleLen::Constant(6)));
        o.insert(0x1E, Opcode(Instruction::Asl, AddrMode::AbsoluteX, CycleLen::Constant(7)));

        o.insert(0x90, Opcode(Instruction::Bcc, AddrMode::Relative, CycleLen::Branch));

        o.insert(0xB0, Opcode(Instruction::Bcs, AddrMode::Relative, CycleLen::Branch));

        o.insert(0xF0, Opcode(Instruction::Beq, AddrMode::Relative, CycleLen::Branch));

        o.insert(0x24, Opcode(Instruction::Bit, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0x2C, Opcode(Instruction::Bit, AddrMode::Absolute, CycleLen::Constant(4)));

        o.insert(0x30, Opcode(Instruction::Bmi, AddrMode::Relative, CycleLen::Branch));

        o.insert(0xD0, Opcode(Instruction::Bne, AddrMode::Relative, CycleLen::Branch));

        o.insert(0x10, Opcode(Instruction::Bpl, AddrMode::Relative, CycleLen::Branch));

        o.insert(0x00, Opcode(Instruction::Brk, AddrMode::Implicit, CycleLen::Constant(7)));

        o.insert(0x50, Opcode(Instruction::Bvc, AddrMode::Relative, CycleLen::Branch));

        o.insert(0x70, Opcode(Instruction::Bvs, AddrMode::Relative, CycleLen::Branch));

        o.insert(0x18, Opcode(Instruction::Clc, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0xD8, Opcode(Instruction::Cld, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0x58, Opcode(Instruction::Cli, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0xB8, Opcode(Instruction::Clv, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0xC9, Opcode(Instruction::Cmp, AddrMode::Immediate, CycleLen::Constant(2)));
        o.insert(0xC5, Opcode(Instruction::Cmp, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0xD5, Opcode(Instruction::Cmp, AddrMode::ZeroPageX, CycleLen::Constant(4)));
        o.insert(0xCD, Opcode(Instruction::Cmp, AddrMode::Absolute, CycleLen::Constant(4)));
        o.insert(0xDD, Opcode(Instruction::Cmp, AddrMode::AbsoluteX, CycleLen::PageCrossed(4)));
        o.insert(0xD9, Opcode(Instruction::Cmp, AddrMode::AbsoluteY, CycleLen::PageCrossed(4)));
        o.insert(0xC1, Opcode(Instruction::Cmp, AddrMode::IndirectX, CycleLen::Constant(6)));
        o.insert(0xD1, Opcode(Instruction::Cmp, AddrMode::IndirectY, CycleLen::PageCrossed(5)));

        o.insert(0xE0, Opcode(Instruction::Cpx, AddrMode::Immediate, CycleLen::Constant(2)));
        o.insert(0xE4, Opcode(Instruction::Cpx, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0xEC, Opcode(Instruction::Cpx, AddrMode::Absolute, CycleLen::Constant(4)));

        o.insert(0xC0, Opcode(Instruction::Cpy, AddrMode::Immediate, CycleLen::Constant(2)));
        o.insert(0xC4, Opcode(Instruction::Cpy, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0xCC, Opcode(Instruction::Cpy, AddrMode::Absolute, CycleLen::Constant(4)));

        o.insert(0xC6, Opcode(Instruction::Dec, AddrMode::ZeroPage, CycleLen::Constant(5)));
        o.insert(0xD6, Opcode(Instruction::Dec, AddrMode::ZeroPageX, CycleLen::Constant(6)));
        o.insert(0xCE, Opcode(Instruction::Dec, AddrMode::Absolute, CycleLen::Constant(6)));
        o.insert(0xDE, Opcode(Instruction::Dec, AddrMode::AbsoluteX, CycleLen::Constant(7)));

        o.insert(0xCA, Opcode(Instruction::Dex, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0x88, Opcode(Instruction::Dey, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0x49, Opcode(Instruction::Eor, AddrMode::Immediate, CycleLen::Constant(2)));
        o.insert(0x45, Opcode(Instruction::Eor, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0x55, Opcode(Instruction::Eor, AddrMode::ZeroPageX, CycleLen::Constant(4)));
        o.insert(0x4D, Opcode(Instruction::Eor, AddrMode::Absolute, CycleLen::Constant(4)));
        o.insert(0x5D, Opcode(Instruction::Eor, AddrMode::AbsoluteX, CycleLen::PageCrossed(4)));
        o.insert(0x59, Opcode(Instruction::Eor, AddrMode::AbsoluteY, CycleLen::PageCrossed(4)));
        o.insert(0x41, Opcode(Instruction::Eor, AddrMode::IndirectX, CycleLen::Constant(6)));
        o.insert(0x51, Opcode(Instruction::Eor, AddrMode::IndirectY, CycleLen::PageCrossed(5)));

        o.insert(0xE6, Opcode(Instruction::Inc, AddrMode::ZeroPage, CycleLen::Constant(5)));
        o.insert(0xF6, Opcode(Instruction::Inc, AddrMode::ZeroPageX, CycleLen::Constant(6)));
        o.insert(0xEE, Opcode(Instruction::Inc, AddrMode::ZeroPage, CycleLen::Constant(6)));
        o.insert(0xFE, Opcode(Instruction::Inc, AddrMode::ZeroPage, CycleLen::Constant(7)));

        o.insert(0xE8, Opcode(Instruction::Inx, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0xC8, Opcode(Instruction::Iny, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0x4C, Opcode(Instruction::Jmp, AddrMode::Absolute, CycleLen::Constant(3)));
        o.insert(0x6C, Opcode(Instruction::Jmp, AddrMode::Indirect, CycleLen::Constant(5)));

        o.insert(0x20, Opcode(Instruction::Jsr, AddrMode::Absolute, CycleLen::Constant(6)));

        o.insert(0xA9, Opcode(Instruction::Lda, AddrMode::Immediate, CycleLen::Constant(2)));
        o.insert(0xA5, Opcode(Instruction::Lda, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0xB5, Opcode(Instruction::Lda, AddrMode::ZeroPageX, CycleLen::Constant(4)));
        o.insert(0xAD, Opcode(Instruction::Lda, AddrMode::Absolute, CycleLen::Constant(4)));
        o.insert(0xBD, Opcode(Instruction::Lda, AddrMode::AbsoluteX, CycleLen::PageCrossed(4)));
        o.insert(0xB9, Opcode(Instruction::Lda, AddrMode::AbsoluteY, CycleLen::PageCrossed(4)));
        o.insert(0xA1, Opcode(Instruction::Lda, AddrMode::IndirectX, CycleLen::Constant(6)));
        o.insert(0xB1, Opcode(Instruction::Lda, AddrMode::IndirectY, CycleLen::PageCrossed(5)));

        o.insert(0xA2, Opcode(Instruction::Ldx, AddrMode::Immediate, CycleLen::Constant(2)));
        o.insert(0xA6, Opcode(Instruction::Ldx, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0xB6, Opcode(Instruction::Ldx, AddrMode::ZeroPageY, CycleLen::Constant(4)));
        o.insert(0xAE, Opcode(Instruction::Ldx, AddrMode::Absolute, CycleLen::Constant(4)));
        o.insert(0xBE, Opcode(Instruction::Ldx, AddrMode::AbsoluteY, CycleLen::PageCrossed(4)));

        o.insert(0xA0, Opcode(Instruction::Ldy, AddrMode::Immediate, CycleLen::Constant(2)));
        o.insert(0xA4, Opcode(Instruction::Ldy, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0xB4, Opcode(Instruction::Ldy, AddrMode::ZeroPageX, CycleLen::Constant(4)));
        o.insert(0xAC, Opcode(Instruction::Ldy, AddrMode::Absolute, CycleLen::Constant(4)));
        o.insert(0xBC, Opcode(Instruction::Ldy, AddrMode::AbsoluteX, CycleLen::PageCrossed(4)));

        o.insert(0x4A, Opcode(Instruction::Lsr, AddrMode::Accumulator, CycleLen::Constant(2)));
        o.insert(0x46, Opcode(Instruction::Lsr, AddrMode::ZeroPage, CycleLen::Constant(5)));
        o.insert(0x56, Opcode(Instruction::Lsr, AddrMode::ZeroPageX, CycleLen::Constant(6)));
        o.insert(0x4E, Opcode(Instruction::Lsr, AddrMode::Absolute, CycleLen::Constant(6)));
        o.insert(0x5E, Opcode(Instruction::Lsr, AddrMode::AbsoluteX, CycleLen::Constant(7)));

        o.insert(0xEA, Opcode(Instruction::Nop, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0x09, Opcode(Instruction::Ora, AddrMode::Immediate, CycleLen::Constant(2)));
        o.insert(0x05, Opcode(Instruction::Ora, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0x15, Opcode(Instruction::Ora, AddrMode::ZeroPageX, CycleLen::Constant(4)));
        o.insert(0x0D, Opcode(Instruction::Ora, AddrMode::Absolute, CycleLen::Constant(4)));
        o.insert(0x1D, Opcode(Instruction::Ora, AddrMode::AbsoluteX, CycleLen::Constant(4)));
        o.insert(0x19, Opcode(Instruction::Ora, AddrMode::AbsoluteY, CycleLen::PageCrossed(4)));
        o.insert(0x01, Opcode(Instruction::Ora, AddrMode::IndirectX, CycleLen::Constant(6)));
        o.insert(0x11, Opcode(Instruction::Ora, AddrMode::IndirectY, CycleLen::PageCrossed(5)));

        o.insert(0x48, Opcode(Instruction::Pha, AddrMode::Implicit, CycleLen::Constant(3)));

        o.insert(0x08, Opcode(Instruction::Php, AddrMode::Implicit, CycleLen::Constant(3)));

        o.insert(0x68, Opcode(Instruction::Pla, AddrMode::Implicit, CycleLen::Constant(3)));

        o.insert(0x28, Opcode(Instruction::Plp, AddrMode::Implicit, CycleLen::Constant(3)));

        o.insert(0x2A, Opcode(Instruction::Rol, AddrMode::Accumulator, CycleLen::Constant(2)));
        o.insert(0x26, Opcode(Instruction::Rol, AddrMode::ZeroPage, CycleLen::Constant(5)));
        o.insert(0x36, Opcode(Instruction::Rol, AddrMode::ZeroPageX, CycleLen::Constant(6)));
        o.insert(0x2E, Opcode(Instruction::Rol, AddrMode::Absolute, CycleLen::Constant(6)));
        o.insert(0x3E, Opcode(Instruction::Rol, AddrMode::AbsoluteX, CycleLen::Constant(7)));

        o.insert(0x6A, Opcode(Instruction::Ror, AddrMode::Accumulator, CycleLen::Constant(2)));
        o.insert(0x66, Opcode(Instruction::Ror, AddrMode::ZeroPage, CycleLen::Constant(5)));
        o.insert(0x76, Opcode(Instruction::Ror, AddrMode::ZeroPageX, CycleLen::Constant(6)));
        o.insert(0x6E, Opcode(Instruction::Ror, AddrMode::Absolute, CycleLen::Constant(6)));
        o.insert(0x7E, Opcode(Instruction::Ror, AddrMode::AbsoluteX, CycleLen::Constant(7)));

        o.insert(0x40, Opcode(Instruction::Rti, AddrMode::Implicit, CycleLen::Constant(6)));

        o.insert(0x60, Opcode(Instruction::Rts, AddrMode::Implicit, CycleLen::Constant(6)));

        o.insert(0xE9, Opcode(Instruction::Sbc, AddrMode::Immediate, CycleLen::Constant(2)));
        o.insert(0xE5, Opcode(Instruction::Sbc, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0xF5, Opcode(Instruction::Sbc, AddrMode::ZeroPageX, CycleLen::Constant(4)));
        o.insert(0xED, Opcode(Instruction::Sbc, AddrMode::Absolute, CycleLen::Constant(4)));
        o.insert(0xFD, Opcode(Instruction::Sbc, AddrMode::AbsoluteX, CycleLen::PageCrossed(4)));
        o.insert(0xF9, Opcode(Instruction::Sbc, AddrMode::AbsoluteY, CycleLen::PageCrossed(4)));
        o.insert(0xE1, Opcode(Instruction::Sbc, AddrMode::IndirectX, CycleLen::Constant(6)));
        o.insert(0xF1, Opcode(Instruction::Sbc, AddrMode::IndirectY, CycleLen::PageCrossed(5)));

        o.insert(0x38, Opcode(Instruction::Sec, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0xF8, Opcode(Instruction::Sed, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0x78, Opcode(Instruction::Sei, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0x85, Opcode(Instruction::Sta, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0x8d, Opcode(Instruction::Sta, AddrMode::Absolute, CycleLen::Constant(4)));

        o.insert(0x86, Opcode(Instruction::Stx, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0x96, Opcode(Instruction::Stx, AddrMode::ZeroPageY, CycleLen::Constant(4)));
        o.insert(0x8E, Opcode(Instruction::Stx, AddrMode::Absolute, CycleLen::Constant(4)));

        o.insert(0x84, Opcode(Instruction::Sty, AddrMode::ZeroPage, CycleLen::Constant(3)));
        o.insert(0x94, Opcode(Instruction::Sty, AddrMode::ZeroPageX, CycleLen::Constant(4)));
        o.insert(0x8C, Opcode(Instruction::Sty, AddrMode::Absolute, CycleLen::Constant(4)));

        o.insert(0xAA, Opcode(Instruction::Tax, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0xA8, Opcode(Instruction::Tay, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0xBA, Opcode(Instruction::Tsx, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0x8A, Opcode(Instruction::Txa, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0x9A, Opcode(Instruction::Txs, AddrMode::Implicit, CycleLen::Constant(2)));

        o.insert(0x98, Opcode(Instruction::Tya, AddrMode::Implicit, CycleLen::Constant(2)));

        o
    };
}
