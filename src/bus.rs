use std::usize;

use crate::cpu::*;

pub struct Bus {
    ram: [u8; 0x0800],
    program_rom: [u8; 0x8000],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            ram: [0; 0x0800],
            program_rom: [0; 0x8000],
        }
    }

    pub fn with_program_rom(program_rom: [u8; 0x8000]) -> Self {
        Self {
            ram: [0; 0x0800],
            program_rom,
        }
    }

    pub fn set_program_rom(&mut self, program_rom: [u8; 0x8000]) {
        self.program_rom = program_rom;
    }

    pub fn read(&self, addr: MemLocation) -> u8 {
        match addr.0 {
            // internal ram
            0..=0x07FF => self.ram[addr.0 as usize],
            // mirror internal ram
            0x0800..=0x0FFF => self.ram[addr.0 as usize - 0x0800],
            0x1000..=0x17FF => self.ram[addr.0 as usize - 0x1000],
            0x1800..=0x1FFF => self.ram[addr.0 as usize - 0x1800],
            // PPU registers
            0x2000..=0x2007 => 0,
            // mirror PPU registers
            0x2008..=0x3FFF => 0,
            // APU registers
            0x4000..=0x4017 => 0,
            // Not normally used
            0x4018..=0x401F => 0,
            // cartridge dependant
            0x4020..=0x5FFF => 0,
            // usually cartridge ram, when present
            0x6000..=0x7FFF => 0,
            // usally cartridge rom
            0x8000..=0xFFFF => self.program_rom[addr.0 as usize - 0x8000],
        }
    }

    pub fn write(&mut self, addr: MemLocation, value: u8) {
        match addr.0 {
            // internal ram
            0..=0x07FF => self.ram[addr.0 as usize] = value,
            // mirror internal ram
            0x0800..=0x0FFF => self.ram[addr.0 as usize - 0x0800] = value,
            0x1000..=0x17FF => self.ram[addr.0 as usize - 0x1000] = value,
            0x1800..=0x1FFF => self.ram[addr.0 as usize - 0x1800] = value,
            // unimplemented
            0x2000..=0x401F => (),
            // cartridge dependant
            0x4020..=0x5FFF => (),
            // usually cartridge ram, when present
            0x6000..=0x7FFF => (),
            // usally cartridge rom
            0x8000..=0xFFFF => (),
        }
    }
}

/// Stores a big-endian 16-bit memory address
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct MemLocation(pub u16);

impl MemLocation {
    pub fn page_0(location: u8) -> Self {
        Self(location as u16)
    }

    pub fn stack(location: u8) -> Self {
        Self(location as u16 + 0x0100)
    }

    pub fn from_little_endian(a: u8, b: u8) -> Self {
        Self(little_endian_to_big_endian(a, b))
    }
}
