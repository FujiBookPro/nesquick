pub struct Ppu {
    w: bool,
    ctrl: u8,
    mask: u8,
    status: u8,
    oam_addr: u8,
    vram_addr: u16,
    oam: [u8; 256],
    memory: PpuMemory,
}

#[derive(Debug)]
pub enum PpuRegister {
    PpuCtrl,
    PpuMask,
    PpuStatus,
    OamAddr,
    OamData,
    PpuScroll,
    PpuAddr,
    PpuData,
    OamDma,
}

/// [line from top][pixel from left]
pub struct Frame([[u8; 240]; 256]);

impl Ppu {
    pub fn new(character_rom: [u8; 0x2000]) -> Self {
        Self {
            w: false,
            ctrl: 0,
            mask: 0,
            status: 0,
            oam_addr: 0,
            vram_addr: 0,
            oam: [0; 256],
            memory: PpuMemory::new(character_rom),
        }
    }
    pub fn draw_frame(&self) -> Frame {
        let mut frame = Frame([[0x1d; 240]; 256]);

        // FIXME: Render background

        // Render sprites
        // FIXME: Correctly render sprites
        let mut secondary_oam = [0; 32];

        for scanline in 0..240 {
            // Fetch the first 8 sprites that appear on this scanline to the secondary oam
            let mut sec_oam_index = 0;
            for i in 0..64 {
                let y = self.oam[i * 4];
                if y > scanline - 8 && y <= scanline {
                    secondary_oam[sec_oam_index] = self.oam[i * 4];
                    secondary_oam[sec_oam_index + 1] = self.oam[i * 4 + 1];
                    secondary_oam[sec_oam_index + 2] = self.oam[i * 4 + 2];
                    secondary_oam[sec_oam_index + 3] = self.oam[i * 4 + 3];

                    sec_oam_index += 1;

                    if sec_oam_index == 8 {
                        break;
                    }
                }
            }

            // Render sprite pixels
            for sprite in 0..sec_oam_index {
                for pixel in 0..8 {
                    let pattern_x = pixel;
                    let pattern_y = scanline - secondary_oam[sprite];

                    // pattern table index stored in byte 1
                    let pattern_index = secondary_oam[sprite + 1];
                    // sprites use palettes 4-7, stored in the first two bits of byte 2
                    let palette = (secondary_oam[sprite + 2] & 0b11) + 4;

                    let color = self.evalueate_pixel(pattern_index, palette, pattern_x, pattern_y);

                    // screen x: (sprite x stored in byte 3) + (current pixel)
                    frame.0[scanline as usize]
                        [pixel as usize + secondary_oam[sprite + 3] as usize] = color;
                }
            }
        }

        frame
    }

    fn evalueate_pixel(&self, pattern_index: u8, palette: u8, x: u8, y: u8) -> u8 {
        0x00
    }

    pub fn write_register(&mut self, reg: PpuRegister, value: u8) {
        match reg {
            PpuRegister::PpuCtrl => self.ctrl = value,
            PpuRegister::PpuMask => self.mask = value,
            PpuRegister::OamAddr => self.oam_addr = value,
            PpuRegister::OamData => {
                self.oam[self.oam_addr as usize] = value;
                self.oam_addr += 1;
            }
            PpuRegister::PpuScroll => unimplemented!(),
            PpuRegister::PpuAddr => {
                if !self.w {
                    self.vram_addr = 0;
                    self.vram_addr |= (value as u16) << 8;
                } else {
                    self.vram_addr |= value as u16;
                }

                self.w = !self.w;

                if self.vram_addr > 0x3fff {
                    self.vram_addr = 0x3fff;
                }
            }
            PpuRegister::PpuData => {
                self.memory.write(self.vram_addr, value);
            }
            PpuRegister::OamDma => unimplemented!(),
            _ => panic!("Write to invalid PPU register {:?}", reg),
        }
    }

    pub fn read_register(&mut self, reg: PpuRegister) -> u8 {
        match reg {
            PpuRegister::PpuStatus => {
                self.w = false;
                self.status
            }
            PpuRegister::OamData => self.oam[self.oam_addr as usize],
            PpuRegister::PpuData => self.memory.read(self.vram_addr),
            _ => panic!("Read from invalid PPU register {:?}", reg),
        }
    }
}

struct PpuMemory {
    character_rom: [u8; 0x2000],
    pallete_ram: [u8; 0x0020],
}

impl PpuMemory {
    pub fn new(character_rom: [u8; 0x2000]) -> Self {
        Self {
            character_rom,
            pallete_ram: [0; 0x0020],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x0fff => self.character_rom[addr as usize], // pattern table 0
            0x1000..=0x1fff => self.character_rom[addr as usize], // pattern table 1
            // FIXME: Handle nametables correctly
            0x3f00..=0x3f1f => self.pallete_ram[addr as usize - 0x3f00],
            _ => panic!("Read from invalid PPU memory address {:x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x0fff => (), // pattern table 0; ROM
            0x1000..=0x1fff => (), // pattern table 1; ROM
            // FIXME: Handle nametables correctly
            0x3f00..=0x3f1f => self.pallete_ram[addr as usize - 0x3f00] = value,
            _ => panic!("Write to invalid PPU memory address {:x}", addr),
        }
    }
}
