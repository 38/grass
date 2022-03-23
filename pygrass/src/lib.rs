use grass_driver::JobDefinition;
use pyo3::{prelude::*, exceptions::PyRuntimeError};

#[pyfunction]
fn execute_job(s: &str) -> PyResult<()> {
    match serde_json::from_str::<JobDefinition>(s) {
        Ok(mut job) => {
            grass_driver::execute_job(&mut job).map_err(|e|
                PyRuntimeError::new_err(format!("RustError: {}", e))
            )
        },
        Err(e) => Err(PyRuntimeError::new_err(format!("IRParsingError: {}", e))),
    }
}

#[pyfunction]
fn expand_macro(s: &str) -> PyResult<()> {
    match serde_json::from_str::<JobDefinition>(s) {
        Ok(mut job) => {
            job.print_expanded_code().map_err(|e|
                PyRuntimeError::new_err(format!("RustError: {}", e))
            )
        },
        Err(e) => Err(PyRuntimeError::new_err(format!("IRParsingError: {}", e))),
    }
}

#[pyfunction]
fn build_job(s: &str) -> PyResult<String> {
    match serde_json::from_str::<JobDefinition>(s) {
        Ok(mut job) => {
            Ok(job.build_artifact().map_err(|e|
                PyRuntimeError::new_err(format!("RustError: {}", e))
            )?.to_string_lossy().into_owned())
        },
        Err(e) => Err(PyRuntimeError::new_err(format!("IRParsingError: {}", e))),
    }
}

#[pyfunction]
fn build_job_and_copy(s: &str, dest: &str) -> PyResult<()> {
    match serde_json::from_str::<JobDefinition>(s) {
        Ok(mut job) => {
            let binary = job.build_artifact().map_err(|e|
                PyRuntimeError::new_err(format!("RustError: {}", e))
            )?;
            std::fs::copy(binary, dest)?;
            Ok(())
        },
        Err(e) => Err(PyRuntimeError::new_err(format!("IRParsingError: {}", e))),
    }
}

#[pyfunction]
fn create_code_compilation_dir(s: &str) -> PyResult<()> {
    match serde_json::from_str::<JobDefinition>(s) {
        Ok(mut job) => {
            let dir = job.get_compilation_dir().unwrap();
            println!("Rust package has been created under {}", dir.to_string_lossy());
            std::process::exit(0);
        },
        Err(e) => Err(PyRuntimeError::new_err(format!("IRParsingError: {}", e))),
    }
}

#[pymodule]
pub fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    env_logger::init();
    m.add_function(wrap_pyfunction!(execute_job, m)?)?;
    m.add_function(wrap_pyfunction!(expand_macro, m)?)?;
    m.add_function(wrap_pyfunction!(create_code_compilation_dir, m)?)?;
    m.add_function(wrap_pyfunction!(build_job, m)?)?;
    m.add_function(wrap_pyfunction!(build_job_and_copy, m)?)?;
    Ok(())
}
