//! Converter between PyTypes and Rust types
use pyo3::{
    types::{PyAny, PyString},
    PyErr,
};
use std::marker::Sized;

pub trait PymTypes<A> {
    fn pr(a: PyAny) -> Result<Self, PyErr>
    where
        Self: Sized;
}

// PymTypes for String
impl PymTypes<String> for String {
    fn pr(a: PyAny) -> Result<Self, PyErr> {
        let s = a.downcast_ref::<PyString>()?;
        Ok(s.to_string()?.to_string())
    }
}

// // PymTypes for bool
// impl PymTypes<PyBool> for bool {
//     fn rp<'a>(self, m: Python<'a>) -> &'a PyBool {
//         PyBool::new(m, self)
//     }
//
//     fn pr(p: PyBool) -> Result<Self, PyErr> {
//         Ok(p.is_true())
//     }
// }
