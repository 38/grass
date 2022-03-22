use std::{fs::File, process::exit};

use grass_driver::JobDefinition;

fn print_usage() -> ! {
    eprintln!("grass-driver exec <job-file>");
    eprintln!("grass-driver expand <job-file>");
    exit(1);
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args: Vec<_> = std::env::args().skip(1).collect();

    if args.len() != 2 {
        print_usage();
    } else {
        let mut job: JobDefinition = serde_json::from_reader(File::open(&args[1])?)?;
        match args[0].as_str() {
            "exec" => {
                grass_driver::execute_job(&mut job)?;
            }
            "expand" => {
                job.print_expanded_code()?;
            }
            _ => {
                drop(job);
                print_usage();
            }
        }
    }
    Ok(())
}
