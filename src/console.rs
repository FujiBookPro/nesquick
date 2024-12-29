use std::cell::RefCell;
use std::rc::Rc;

use crate::{bus::Bus, cpu::Cpu, ppu::Ppu};

pub struct Console {
    pub cpu: Cpu,
    pub ppu: Rc<RefCell<Ppu>>,
}

impl Console {
    pub fn new(game: Game) -> Self {
        let ppu = Rc::new(RefCell::new(Ppu::new()));
        let cpu = Cpu::new(Bus::new(game.program_rom, Rc::clone(&ppu)));
        Self { cpu, ppu }
    }

    /// Decode and run `n` instructions
    pub fn run_steps(&mut self, n: usize) {
        for _ in 0..n {
            self.cpu.step();
        }
    }

    pub fn run_continuous(&mut self) {
        loop {
            self.cpu.step();
        }
    }
}

pub struct Game {
    pub program_rom: [u8; 0x8000],
    // character_rom
    // cartridge ram?
}
