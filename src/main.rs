use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

use clap::Parser;

mod backend;
mod frontend;

#[derive(Parser)]
#[command(author, about, version)]
struct Args {
    /// Chip-8 source file to read
    file: String,
}

fn main() {
    let args = Args::parse();
    let filename = args.file;
    let file_exists = Path::new(&filename).exists();

    if !file_exists {
        println!("ERROR: Couldn't find file: {}", filename);
        exit(exitcode::USAGE);
    }

    let mut chip8 = backend::Chip8::new();
    let mut rom = File::open(filename).expect("ERROR: Couldn't open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    chip8.load(&buffer);

    frontend::run_game(chip8);
}
