fn main() -> Result<(), Box<dyn std::error::Error>> {
    grass_macro::import_grass_ir_from_file!("../data/ir/expand-interval.py.json");
    Ok(())
}