use pyo3::prelude::*;

pub mod lingua_bindings;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn lingua_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<lingua_bindings::builder::PyLanguageDetectorBuilder>()?;
    m.add_class::<lingua_bindings::detector::PyLanguageDetector>()?;
    m.add_class::<lingua_bindings::language::PyLanguage>()?;
    Ok(())
}

