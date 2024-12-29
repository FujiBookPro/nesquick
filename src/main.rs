mod bus;
mod console;
mod cpu;
mod file;
mod opcode;
mod ppu;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let game = crate::file::read_ines_file(&args[1]).unwrap();

    let mut nes = crate::console::Console::new(game);
    println!("{}", nes.cpu);

    nes.run_steps(10);
}
