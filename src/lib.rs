use std::path::PathBuf;

use pyo3::prelude::*;

#[pyclass]
struct CubeData {
    #[pyo3(get)]
    atoms: Vec<u8>,
}

/// Read a `.cube` file.
#[pyfunction]
fn read_cube(path: PathBuf) -> PyResult<CubeData> {
    Ok(CubeData { atoms: vec![] })
}

/// A Python module implemented in Rust.
#[pymodule]
fn flour(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_cube, m)?)?;
    Ok(())
}

