use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

struct Cpu {
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            //TODO
        }
    }
}

fn main() {
    let mut args = env::args();
    let pif_filename = args.nth(1).unwrap();
    let rom_filename = args.nth(2).unwrap();
    let pif_rom = load_bin(pif_filename);
    let rom = load_bin(rom_filename);

    //let mut cpu = Cpu::new();
}

fn load_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer).unwrap();
    file_buffer
}
    
