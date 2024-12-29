pub struct Ppu {
    w: bool,
    ctrl: u8,
    mask: u8,
    status: u8,
    oam_addr: u8,
    vram_addr: u16,
    oam: [u8; 256],
    pallete_ram: [u8; 0x1f],
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

pub struct Frame;

impl Ppu {
    pub fn new() -> Self {
        Self {
            w: false,
            ctrl: 0,
            mask: 0,
            status: 0,
            oam_addr: 0,
            vram_addr: 0,
            oam: [0; 256],
            pallete_ram: [0; 0x1f],
        }
    }
    pub fn draw_frame(&self) -> Frame {
        Frame
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
                if (0x3f00..=0x3f1f).contains(&self.vram_addr) {
                    self.pallete_ram[self.vram_addr as usize - 0x3f00] = value;
                }
                // FIXME: Implement PPU cartridge address space
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
            PpuRegister::PpuData => self.pallete_ram[self.vram_addr as usize - 0x3f00],
            _ => panic!("Read from invalid PPU register {:?}", reg),
        }
    }
}
