use pyo3::types::{PyAny, PyTuple};

/// The core trait of pym.
pub trait Pym {
    fn run(&self, args: PyTuple) -> PyAny;
}
