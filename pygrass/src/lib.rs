use grass_driver::JobDefinition;
use pyo3::{prelude::*, exceptions::PyRuntimeError};

#[pyfunction]
fn execute_job(s: &str) -> PyResult<()> {
    match serde_json::from_str::<JobDefinition>(s) {
        Ok(mut job) => {
            grass_driver::execute_job(&mut job).map_err(|e|
                PyRuntimeError::new_err(format!("{}", e))
            )
        },
        Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
    }
}

#[pyfunction]
fn expand_macro(s: &str) -> PyResult<()> {
    match serde_json::from_str::<JobDefinition>(s) {
        Ok(mut job) => {
            job.print_expanded_code().map_err(|e|
                PyRuntimeError::new_err(format!("{}", e))
            )
        },
        Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
    }
}

#[pymodule]
pub fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    env_logger::init();
    m.add_function(wrap_pyfunction!(execute_job, m)?)?;
    m.add_function(wrap_pyfunction!(expand_macro, m)?)?;
    Ok(())
}