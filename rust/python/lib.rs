use pyo3::prelude::*;

mod example;

pub use example::Example;


#[pymodule]
fn atomic_counter(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // Example
    m.add_class::<Example>().unwrap();
    Ok(())
}
