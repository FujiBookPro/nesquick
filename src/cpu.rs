use crate::opcode::AddrMode;
use crate::opcode::Instruction;
use crate::opcode::Opcode;

pub struct Cpu {
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub status: CpuStatus,
    pub stack_pointer: u8,
    pub memory: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            accumulator: 0,
            x: 0,
            y: 0,
            pc: 0,
            status: CpuStatus::new(),
            stack_pointer: 0,
            memory: vec![0; 0xffff],
        }
    }

    pub fn load_program(&mut self, program: &[u8], location: u16) {
        for (i, byte) in program.iter().enumerate() {
            self.memory[i + location as usize] = *byte;
        }
    }

    pub fn run(&mut self, start_address: u16) {
        self.pc = start_address;

        loop {
            let code = self.pc_next();
            let opcode = Opcode::decode(code);
            if let Some(opcode) = opcode {

                if opcode.0 == Instruction::Brk {
                    break;
                }
                self.run_instruction(opcode);

                println!("A: {:x}, PC: {:x} 0x02: {:x} 0x05: {:x}", self.accumulator, self.pc, self.memory[0x02], self.memory[0x05]);
            } else {
                panic!("Unimplimented opcode {:x}", code);
            }
        }
    }

    fn run_instruction(&mut self, opcode: Opcode) {
        let Opcode(instruction, addr_mode, _, _) = opcode;

        match instruction {
            Instruction::Adc => {
                let value = self.get_value(&addr_mode);
                self.add_accumulator(value);
            }
            Instruction::And => {
                let value = self.get_value(&addr_mode);
                self.and_accumulator(value);
            }
            Instruction::Asl => {
                if addr_mode == AddrMode::Accumulator {
                    self.shift_left_accumulator();
                } else {
                    let location = self.get_location(&addr_mode);
                    self.shift_left_memory(location);
                }
            }
            Instruction::Brk => unreachable!(), // FIXME: Handle BRK more cleanly
            Instruction::Clc => self.status.set_carry(false),
            Instruction::Cld => self.status.set_decimal(false),
            Instruction::Cli => self.status.set_int_disable(false),
            Instruction::Clv => self.status.set_overflow(false),
            Instruction::Dec => {
                let location = self.get_location(&addr_mode);
                self.dec_memory(location);
            }
            Instruction::Dex => self.dec_register(Register::X),
            Instruction::Dey => self.dec_register(Register::Y),
            Instruction::Eor => {
                let value = self.get_value(&addr_mode);
                self.xor_accumulator(value);
            }
            Instruction::Inc => {
                let location = self.get_location(&addr_mode);
                self.inc_memory(location);
            }
            Instruction::Inx => self.inc_register(Register::X),
            Instruction::Iny => self.inc_register(Register::Y),
            Instruction::Lda => {
                let value = self.get_value(&addr_mode);
                self.load(Register::A, value);
            }
            Instruction::Ldx => {
                let value = self.get_value(&addr_mode);
                self.load(Register::X, value);
            }
            Instruction::Ldy => {
                let value = self.get_value(&addr_mode);
                self.load(Register::Y, value);
            }
            Instruction::Lsr => {
                if addr_mode == AddrMode::Accumulator {
                    self.shift_right_accumulator();
                } else {
                    let location = self.get_location(&addr_mode);
                    self.shift_right_memory(location);
                }
            }
            Instruction::Nop => {}
            Instruction::Ora => {
                let value = self.get_value(&addr_mode);
                self.or_accumulator(value);
            }
            Instruction::Rol => {
                if addr_mode == AddrMode::Accumulator {
                    self.rotate_left_accumulator();
                } else {
                    let location = self.get_location(&addr_mode);
                    self.rotate_left_memory(location);
                }
            }
            Instruction::Ror => {
                if addr_mode == AddrMode::Accumulator {
                    self.rotate_right_accumulator();
                } else {
                    let location = self.get_location(&addr_mode);
                    self.rotate_right_memory(location);
                }
            }
            Instruction::Sta => {
                let location = self.get_location(&addr_mode);
                self.store(Register::A, location);
            }
        }
    }

    fn get_value(&mut self, addr_mode: &AddrMode) -> u8 {
        let mut a = 0;
        let mut b = 0;

        if addr_mode.n_param_bytes() >= 1 {
            a = self.pc_next();
        }

        if addr_mode.n_param_bytes() >= 2 {
            b = self.pc_next();
        }

        match addr_mode {
            AddrMode::Immediate => a,
            AddrMode::ZeroPage => self.memory_read(MemLocation::page_0(a)),
            AddrMode::ZeroPageX => self.memory_read(MemLocation::page_0(a + self.x)),
            AddrMode::ZeroPageY => self.memory_read(MemLocation::page_0(a + self.y)),
            AddrMode::Absolute => self.memory_read(MemLocation::from_little_endian(a, b)),
            AddrMode::AbsoluteX => self.memory_read(MemLocation::from_little_endian(a, b)),
            AddrMode::AbsoluteY => self.memory_read(MemLocation::from_little_endian(a, b)),
            AddrMode::IndirectX => {
                let location =
                    MemLocation::page_0(self.memory_read(MemLocation::page_0(a + self.x)));
                self.memory_read(location)
            }
            AddrMode::IndirectY => {
                let location =
                    MemLocation::page_0(self.memory_read(MemLocation::page_0(a)) + self.y);
                self.memory_read(location)
            }
            _ => panic!("Invalid address mode {:?}", addr_mode)
        }
    }

    fn get_location(&mut self, addr_mode: &AddrMode) -> MemLocation {
        let mut a = 0;
        let mut b = 0;

        if addr_mode.n_param_bytes() >= 1 {
            a = self.pc_next();
        }

        if addr_mode.n_param_bytes() >= 2 {
            b = self.pc_next();
        }

        match addr_mode {
            AddrMode::ZeroPage => MemLocation::page_0(a),
            AddrMode::ZeroPageX => MemLocation::page_0(a + self.x),
            AddrMode::ZeroPageY => MemLocation::page_0(a + self.y),
            AddrMode::Absolute => MemLocation::from_little_endian(a, b),
            AddrMode::AbsoluteX => MemLocation::from_little_endian(a, b),
            AddrMode::AbsoluteY => MemLocation::from_little_endian(a, b),
            _ => todo!(),
        }
    }

    fn memory_read(&self, location: MemLocation) -> u8 {
        self.memory[location.0 as usize]
    }

    fn memory_write(&mut self, value: u8, location: MemLocation) {
        self.memory[location.0 as usize] = value;
    }

    fn pc_next(&mut self) -> u8 {
        let value = self.memory[self.pc as usize];
        self.pc += 1;
        value
    }

    fn load(&mut self, reg: Register, value: u8) {
        let reg_ref = match reg {
            Register::A => &mut self.accumulator,
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            _ => panic!("Load into invalid register {:?}", reg),
        };

        *reg_ref = value;

        self.status.set_zero(*reg_ref == 0);
        self.status.set_negative(*reg_ref << 7 == 1);
    }

    fn store(&mut self, reg: Register, location: MemLocation) {
        let value = match reg {
            Register::A => self.accumulator,
            Register::X => self.x,
            Register::Y => self.y,
            _ => panic!("Store from invalid register {:?}", reg),
        };

        self.memory_write(value, location);
    }

    fn dec_memory(&mut self, location: MemLocation) {
        self.memory_write(self.memory_read(location) - 1, location);

        let new_value = self.memory_read(location);
        self.status.set_zero(new_value == 0);
        self.status.set_negative(new_value << 7 == 1);
    }

    fn dec_register(&mut self, reg: Register) {
        let reg_ref = match reg {
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            _ => panic!("Decriment invalid register {:?}", reg),
        };

        *reg_ref -= 1;

        self.status.set_zero(*reg_ref == 0);
        self.status.set_negative(*reg_ref << 7 == 1);
    }

    fn add_accumulator(&mut self, value: u8) {
        // The carry bit is included in the calculation. If it has already been set and the current
        // calculation wraps, the result should wrap, unsetting the bit.
        //
        //   1 11111111
        // +   00000001
        // = 0 00000001
        let carry = (self.accumulator as u16 + value as u16 > 255) ^ self.status.get_carry();
        self.accumulator = self.accumulator.wrapping_add(value);

        if self.status.get_carry() {
            self.accumulator += 1;
        }

        self.status.set_carry(carry);
        self.status.set_zero(self.accumulator == 0);
        // TODO: set overflow flag
        self.status.set_negative(self.accumulator >> 7 == 1);
    }

    fn and_accumulator(&mut self, value: u8) {
        self.accumulator &= value;

        self.status.set_zero(self.accumulator == 0);
        self.status.set_negative(self.accumulator << 7 == 1);
    }

    fn bitshift_and_set_flags(&mut self, value: &mut u8, direction: ShiftDirection) {
        match direction {
            ShiftDirection::Left => {
                self.status.set_carry(*value >> 7 == 1); // bit 7 is stored in carry flag
                *value <<= 1;
            }
            ShiftDirection::Right => {
                self.status.set_carry(*value & 1 == 1); // bit 0 is stored in carry flag
                *value >>= 1;
            }
        }

        self.status.set_zero(*value == 0);
        self.status.set_negative(*value >> 7 == 1);
    }

    fn shift_left_accumulator(&mut self) {
        let mut value = self.accumulator;
        self.bitshift_and_set_flags(&mut value, ShiftDirection::Left);

        self.accumulator = value;
    }

    fn shift_left_memory(&mut self, location: MemLocation) {
        let mut value = self.memory_read(location);
        self.bitshift_and_set_flags(&mut value, ShiftDirection::Left);

        self.memory_write(value, location);
    }

    fn shift_right_accumulator(&mut self) {
        let mut value = self.accumulator;
        self.bitshift_and_set_flags(&mut value, ShiftDirection::Right);

        self.accumulator = value;
    }

    fn shift_right_memory(&mut self, location: MemLocation) {
        let mut value = self.memory_read(location);
        self.bitshift_and_set_flags(&mut value, ShiftDirection::Right);

        self.memory_write(value, location);
    }

    fn rotate_and_set_flags(&mut self, value: &mut u8, direction: ShiftDirection) {
        match direction {
            ShiftDirection::Left => {
                let old_carry_value = if self.status.get_carry() { 1 } else { 0 };

                self.status.set_carry(*value >> 7 == 1); // old bit 7 is stored in carry flag
                *value <<= 1;

                *value |= old_carry_value; // original value of carry flag is placed in bit 0
            }
            ShiftDirection::Right => {
                let old_carry_value = if self.status.get_carry() { 1 } else { 0 };

                self.status.set_carry(*value & 1 == 1); // old bit 1 is stored in carry flag
                *value >>= 1;

                *value |= old_carry_value << 7; // original value of carry flag is placed in bit 7
            }
        }

        self.status.set_zero(self.accumulator == 0);
        self.status.set_negative(self.accumulator << 7 == 1);
    }

    fn rotate_left_accumulator(&mut self) {
        let mut value = self.accumulator;
        self.rotate_and_set_flags(&mut value, ShiftDirection::Left);

        self.accumulator = value;
    }

    fn rotate_left_memory(&mut self, location: MemLocation) {
        let mut value = self.memory_read(location);
        self.rotate_and_set_flags(&mut value, ShiftDirection::Left);

        self.memory_write(value, location);
    }

    fn rotate_right_accumulator(&mut self) {
        let mut value = self.accumulator;
        self.rotate_and_set_flags(&mut value, ShiftDirection::Right);

        self.accumulator = value;
    }

    fn rotate_right_memory(&mut self, location: MemLocation) {
        let mut value = self.memory_read(location);
        self.rotate_and_set_flags(&mut value, ShiftDirection::Right);

        self.memory_write(value, location);
    }

    fn or_accumulator(&mut self, value: u8) {
        self.accumulator |= value;

        self.status.set_zero(self.accumulator == 0);
        self.status.set_negative(self.accumulator << 7 == 1);
    }

    fn xor_accumulator(&mut self, value: u8) {
        self.accumulator ^= value;

        self.status.set_zero(self.accumulator == 0);
        self.status.set_negative(self.accumulator << 7 == 1);
    }

    fn inc_memory(&mut self, location: MemLocation) {
        self.memory_write(self.memory_read(location) + 1, location);

        let new_value = self.memory_read(location);
        self.status.set_zero(new_value == 0);
        self.status.set_negative(new_value << 7 == 1);
    }

    fn inc_register(&mut self, reg: Register) {
        let reg_ref = match reg {
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            _ => panic!("Decriment invalid register {:?}", reg),
        };

        *reg_ref += 1;

        self.status.set_zero(*reg_ref == 0);
        self.status.set_negative(*reg_ref << 7 == 1);
    }
}

enum ShiftDirection {
    Left,
    Right,
}

#[derive(Debug)]
enum Register {
    A,
    X,
    Y,
    PC,
    S,
    P,
}

/// Stores a big-endian 16-bit memory address
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
struct MemLocation(pub u16);

impl MemLocation {
    pub fn page_0(location: u8) -> Self {
        Self(location as u16)
    }

    pub fn from_little_endian(a: u8, b: u8) -> Self {
        let mut location = 0;
        location += (b as u16) << 8;
        location += a as u16;
        Self(location)
    }
}

pub struct CpuStatus {
    byte: u8,
}

impl CpuStatus {
    pub fn new() -> Self {
        Self {
            byte: 1 << 5, // the 5th bit is always set
        }
    }

    pub fn get_register_byte(&self) -> u8 {
        self.byte
    }

    pub fn get_carry(&self) -> bool {
        self.byte & 1 << 0 != 0
    }

    pub fn get_zero(&self) -> bool {
        self.byte & 1 << 1 != 0
    }

    pub fn get_int_disable(&self) -> bool {
        self.byte & 1 << 2 != 0
    }
    pub fn get_decimal(&self) -> bool {
        self.byte & 1 << 3 != 0
    }
    pub fn get_break(&self) -> bool {
        self.byte & 1 << 4 != 0
    }

    pub fn get_overflow(&self) -> bool {
        self.byte & 1 << 6 != 0
    }

    pub fn get_negative(&self) -> bool {
        self.byte & 1 << 7 != 0
    }

    pub fn set_carry(&mut self, value: bool) {
        let n = if value { 1 } else { 0 };
        self.byte &= n;
    }

    pub fn set_zero(&mut self, value: bool) {
        let n = if value { 1 } else { 0 };
        self.byte &= n << 1;
    }

    pub fn set_int_disable(&mut self, value: bool) {
        let n = if value { 1 } else { 0 };
        self.byte &= n << 2;
    }

    pub fn set_decimal(&mut self, value: bool) {
        let n = if value { 1 } else { 0 };
        self.byte &= n << 3;
    }

    pub fn set_break(&mut self, value: bool) {
        let n = if value { 1 } else { 0 };
        self.byte &= n << 4;
    }

    pub fn set_overflow(&mut self, value: bool) {
        let n = if value { 1 } else { 0 };
        self.byte &= n << 6;
    }

    pub fn set_negative(&mut self, value: bool) {
        let n = if value { 1 } else { 0 };
        self.byte &= n << 7;
    }
}
