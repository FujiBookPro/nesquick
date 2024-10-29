use crate::{bus::Bus, cpu::Cpu};

pub struct Console {
    pub cpu: Cpu,
    // PPU
    // RAM
}

impl Console {
    pub fn new(game: Game) -> Self {
        Self {
            cpu: Cpu::new(Bus::with_program_rom(game.program_rom)),
        }
    }
}

pub struct Game {
    pub program_rom: [u8; 0x8000],
    // character_rom
    // cartridge ram?
}
