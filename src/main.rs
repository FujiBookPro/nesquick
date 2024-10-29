mod bus;
mod console;
mod cpu;
mod file;
mod opcode;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let game = crate::file::read_ines_file(&args[1]).unwrap();

    let nes = crate::console::Console::new(game);
    println!("{}", nes.cpu);

    // to-do list:
    // correctly set cpu startup conditions
    // run cpu for given amount of time
    // emit debug info
    // run test rom

    // run game
}
