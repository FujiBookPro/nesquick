use crate::bus::{Bus, MemLocation};
use crate::opcode::{AddrMode, Instruction, Opcode};

pub struct Cpu {
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub status: CpuStatus,
    pub stack_pointer: u8,
    pub bus: Bus,
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
            bus: Bus::new(),
        }
    }

    pub fn load_program(&mut self, program: &[u8], location: MemLocation) {
        self.bus.load_program(program, location);
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

                println!(
                    "A: {:x}, PC: {:x} 0x01: {:x} 0x05: {:x}",
                    self.accumulator,
                    self.pc,
                    self.bus.read(MemLocation::page_0(0x01)),
                    self.bus.read(MemLocation::page_0(0x05))
                );
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
            Instruction::Bcc => self.branch(!self.status.get_carry()),
            Instruction::Bcs => self.branch(self.status.get_carry()),
            Instruction::Beq => self.branch(self.status.get_zero()),
            Instruction::Bmi => self.branch(self.status.get_negative()),
            Instruction::Bne => self.branch(!self.status.get_zero()),
            Instruction::Bpl => self.branch(self.status.get_negative()),
            Instruction::Brk => unreachable!(), // FIXME: Handle BRK more cleanly
            Instruction::Bvc => self.branch(!self.status.get_overflow()),
            Instruction::Bvs => self.branch(self.status.get_overflow()),
            Instruction::Clc => self.status.set_carry(false),
            Instruction::Cld => self.status.set_decimal(false),
            Instruction::Cli => self.status.set_int_disable(false),
            Instruction::Clv => self.status.set_overflow(false),
            Instruction::Cmp => {
                let value = self.get_value(&addr_mode);
                self.cmp(value, self.accumulator);
            }
            Instruction::Cpx => {
                let value = self.get_value(&addr_mode);
                self.cmp(value, self.x);
            }
            Instruction::Cpy => {
                let value = self.get_value(&addr_mode);
                self.cmp(value, self.y);
            }
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
            Instruction::Jmp => self.jump(&addr_mode),
            Instruction::Jsr => self.jump_sub(),
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
            Instruction::Pha => self.stack_push(self.accumulator),
            Instruction::Php => self.stack_push(self.status.byte),
            Instruction::Pla => self.accumulator = self.stack_pop(),
            Instruction::Plp => self.status.byte = self.stack_pop(),
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
            Instruction::Rts => self.ret_sub(),
            Instruction::Sta => {
                let location = self.get_location(&addr_mode);
                self.store(Register::A, location);
            }
            Instruction::Stx => {
                let location = self.get_location(&addr_mode);
                self.store(Register::X, location);
            }
            Instruction::Sty => {
                let location = self.get_location(&addr_mode);
                self.store(Register::Y, location);
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
            AddrMode::Relative => a,
            _ => panic!("Invalid address mode {:?}", addr_mode),
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

    fn jump(&mut self, addr_mode: &AddrMode) {
        let a = self.pc_next();
        let b = self.pc_next();

        let value = little_endian_to_big_endian(a, b);

        self.pc = match addr_mode {
            AddrMode::Absolute => value,
            AddrMode::Implicit => {
                let first = self.bus.read(MemLocation(value));
                let second = self.bus.read(MemLocation(value + 1));
                little_endian_to_big_endian(first, second)
            }
            _ => panic!("Invalid jump addressing mode: {:?}", addr_mode),
        }
    }

    fn jump_sub(&mut self) {
        let a = self.pc_next();
        let b = self.pc_next();

        let value = little_endian_to_big_endian(a, b);

        let return_addr = (self.pc - 1).to_le_bytes();

        self.stack_push(return_addr[0]);
        self.stack_push(return_addr[1]);

        self.pc = value;
    }

    fn ret_sub(&mut self) {
        let a = self.stack_pop();
        let b = self.stack_pop();

        let value = little_endian_to_big_endian(a, b);

        self.pc = value;
    }

    fn memory_read(&self, location: MemLocation) -> u8 {
        self.bus.read(MemLocation(location.0))
    }

    fn memory_write(&mut self, value: u8, location: MemLocation) {
        self.bus.write(MemLocation(location.0), value);
    }

    fn pc_next(&mut self) -> u8 {
        let value = self.bus.read(MemLocation(self.pc));
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

    fn branch(&mut self, should_branch: bool) {
        let value = self.get_value(&AddrMode::Relative);
        let value_i8: i8 = unsafe { std::mem::transmute(value) };
        if should_branch {
            // sure hope this gets optimized
            if value_i8 >= 0 {
                self.pc += value_i8 as u16;
            } else {
                self.pc -= value_i8.abs() as u16;
            }
        }
    }

    fn stack_push(&mut self, value: u8) {
        self.memory_write(value, MemLocation::stack(self.stack_pointer));

        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        self.memory_read(MemLocation::stack(self.stack_pointer))
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

    fn cmp(&mut self, value: u8, register: u8) {
        self.status.set_carry(register >= value);
        self.status.set_zero(register == value);
        self.status.set_negative(value >> 7 == 1);
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

pub fn little_endian_to_big_endian(a: u8, b: u8) -> u16 {
    let mut r = 0;
    r += (b as u16) << 8;
    r += a as u16;
    r
}

enum ShiftDirection {
    Left,
    Right,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Register {
    A,
    X,
    Y,
    PC,
    S,
    P,
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
