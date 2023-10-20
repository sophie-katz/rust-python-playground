use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
/// 
/// This doc comment will be used as the module's docstring.
#[pymodule]
// The name of the Python module (this is what you import):
#[pyo3(name = "rust_python_module")]
fn rust_python_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    register_child_module(py, m)
}

// You can register child modules like this
fn register_child_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "child_module")?;
    child_module.add_function(wrap_pyfunction!(sum_as_string, child_module)?)?;
    parent_module.add_submodule(child_module)
}
