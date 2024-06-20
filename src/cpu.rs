use crate::opcode::AddrMode;
use crate::opcode::Instruction;
use crate::opcode::Opcode;

pub struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    s: u8,
    p: u8,
    memory: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0,
            p: 0,
            memory: vec![0; 0xffff],
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, byte) in program.iter().enumerate() {
            self.memory[i] = *byte;
        }
    }

    pub fn run(&mut self) {
        let should_halt = false;

        while !should_halt {
            println!("{}", self.a);

            let opcode = Opcode::decode(self.pc_next()).expect("Invalid opcode");
            self.run_instruction(opcode);
        }
    }

    fn run_instruction(&mut self, opcode: Opcode) {
        let Opcode(instruction, addr_mode, _) = opcode;

        match instruction {
            Instruction::ADC => {
                let value = self.get_value(&addr_mode);
                self.add_accumulator(value);
            }
            Instruction::LDA => {
                let value = self.get_value(&addr_mode);
                self.load(Register::A, value);
            }
            Instruction::STA => {
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
            _ => todo!(),
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
        return value;
    }

    fn load(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::X => self.x = value,
            Register::Y => self.y = value,
            _ => panic!("Load into invalid register {:?}", reg),
        }
    }

    fn store(&mut self, reg: Register, location: MemLocation) {
        let value;
        match reg {
            Register::A => value = self.a,
            Register::X => value = self.x,
            Register::Y => value = self.y,
            _ => panic!("Store from invalid register {:?}", reg),
        }

        self.memory_write(value, location);
    }

    fn add_accumulator(&mut self, value: u8) {
        self.a += value;
    }
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
