use crate::console::Game;
use std::fs;

pub fn read_ines_file(path: &str) -> Result<Game, String> {
    let file = fs::read(path);
    if let Err(e) = file {
        return Err(e.to_string());
    }
    let file = file.unwrap();

    // decode header (first 16 bytes)
    if file[0..4] != [0x4E, 0x45, 0x53, 0x1A] {
        return Err("Invalid NES file header".to_string());
    }

    let prg_size = file[4];
    let chr_size = file[5];

    let flag_6 = file[6];
    let flag_7 = file[7];
    let flag_8 = file[8];
    let flag_9 = file[9];
    let flag_10 = file[10];
    // bytes 11-16 are unused

    if flag_6 != 1 {
        return Err("Unsupported flag 6".to_string());
    }

    if flag_7 != 0 {
        return Err("Unsupported flag 7".to_string());
    }

    if flag_8 != 0 {
        return Err("Unsupported feature: PRG ram".to_string());
    }

    if flag_9 != 0 {
        return Err("Unsupported feature: PAL video".to_string());
    }

    if flag_10 != 0 {
        return Err("Unsupported flag 10".to_string());
    }

    if prg_size > 2 {
        return Err("Unsupported feature: rom mapping".to_string());
    }

    // copy program rom
    let mut program_rom = [0; 0x8000];
    let end_byte = 0x10 + 0x4000 * prg_size as usize;
    program_rom.clone_from_slice(&file[0x10..end_byte]);

    // copy char rom

    return Ok(Game { program_rom });
}
