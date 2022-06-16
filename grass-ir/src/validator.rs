use std::{env::args, fs::File};

use grass_ir::GrassIR;
use serde_json::from_reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for path in args().skip(1) {
        let fp = File::open(path)?;
        let _ir: GrassIR = from_reader(fp)?;
    }
    Ok(())
}
