mod dependency;
mod job;


fn return_true() -> bool { true }

use std::io::{BufReader, BufRead};

pub use job::JobDefinition;

pub fn execute_job(job: &mut JobDefinition) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = job.execute_artifact() {
        let err_log = BufReader::new(job.get_stderr_log()?);
        for line in err_log.lines() {
            let line_text = line?;
            eprintln!("stderr: {}", line_text);
        }
        Err(e)?;
    }
    Ok(())
}