use std::{fs::File, process::exit, io::{BufReader, BufRead}};

use job::JobDefinition;


mod dependency;
mod job;


pub fn return_true() -> bool { true }

fn print_usage() -> ! {
    eprintln!("grass-driver exec <job-file>");
    eprintln!("grass-driver expand <job-file>");
    exit(1);
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args:Vec<_> = std::env::args().skip(1).collect();

    if args.len() != 2 {
        print_usage();
    } else {
        let mut job : JobDefinition = serde_json::from_reader(File::open(&args[1])?)?;
        match args[0].as_str() {
            "exec" => {
                if let Err(e) = job.execute_artifact() {
                    let err_log = BufReader::new(job.get_stderr_log()?);
                    for line in err_log.lines() {
                        let line_text = line?;
                        eprintln!("stderr: {}", line_text);
                    }
                    Err(e)?;
                }
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
