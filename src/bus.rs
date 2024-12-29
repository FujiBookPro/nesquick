use std::{cell::RefCell, rc::Rc};

use crate::{cpu::*, ppu::Ppu, ppu::PpuRegister};

pub struct Bus {
    ram: [u8; 0x0800],
    program_rom: [u8; 0x8000],
    ppu: Rc<RefCell<Ppu>>,
}

impl Bus {
    pub fn new(program_rom: [u8; 0x8000], ppu: Rc<RefCell<Ppu>>) -> Self {
        Self {
            ram: [0; 0x0800],
            program_rom,
            ppu,
        }
    }

    pub fn read(&mut self, addr: MemLocation) -> u8 {
        match addr.0 {
            // internal ram
            0..=0x07FF => self.ram[addr.0 as usize],
            // mirror internal ram
            0x0800..=0x0FFF => self.ram[addr.0 as usize - 0x0800],
            0x1000..=0x17FF => self.ram[addr.0 as usize - 0x1000],
            0x1800..=0x1FFF => self.ram[addr.0 as usize - 0x1800],
            // PPU registers
            0x2000..=0x2007 => self.handle_ppu_read(addr.0),
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

    fn handle_ppu_read(&mut self, addr: u16) -> u8 {
        match addr {
            // ppuctrl
            0x2000 => 0, // not readable
            // ppumask0
            0x2001 => 0, // not readable
            // ppustatus
            0x2002 => self.ppu.borrow_mut().read_register(PpuRegister::PpuStatus),
            // oamaddr
            0x2003 => 0, // not readable
            // oamdata
            0x2004 => self.ppu.borrow_mut().read_register(PpuRegister::OamData),
            // ppuscroll
            0x2005 => 0, // not readable
            // ppuaddr
            0x2006 => 0, // not readable
            // ppudata
            0x2007 => self.ppu.borrow_mut().read_register(PpuRegister::PpuData),
            // FIXME: add oamdata at 0x4014
            _ => unreachable!(),
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
            // ppu registers
            0x2000..=0x2007 => self.handle_ppu_write(addr.0, value),
            // ppu register mirrors
            0x2008..=0x3FFF => self.handle_ppu_write((addr.0 % 8) + 0x2000, value),
            // unimplemented
            0x4000..=0x401F => (),
            // cartridge dependant
            0x4020..=0x5FFF => (),
            // usually cartridge ram, when present
            0x6000..=0x7FFF => (),
            // usally cartridge rom
            0x8000..=0xFFFF => (),
        }
    }

    fn handle_ppu_write(&mut self, addr: u16, value: u8) {
        match addr {
            // ppuctrl
            0x2000 => self
                .ppu
                .borrow_mut()
                .write_register(PpuRegister::PpuCtrl, value),
            // ppumask
            0x2001 => self
                .ppu
                .borrow_mut()
                .write_register(PpuRegister::PpuMask, value),
            // ppustatus
            0x2002 => (), // not writable
            // oamaddr
            0x2003 => self
                .ppu
                .borrow_mut()
                .write_register(PpuRegister::OamAddr, value),
            // oamdata
            0x2004 => self
                .ppu
                .borrow_mut()
                .write_register(PpuRegister::OamData, value),
            // ppuscroll
            0x2005 => self
                .ppu
                .borrow_mut()
                .write_register(PpuRegister::PpuScroll, value),
            // ppuaddr
            0x2006 => self
                .ppu
                .borrow_mut()
                .write_register(PpuRegister::PpuAddr, value),
            // ppudata
            0x2007 => self
                .ppu
                .borrow_mut()
                .write_register(PpuRegister::PpuData, value),
            // FIXME: add oamdata at 0x4014
            _ => unreachable!(),
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
