use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use std::str::FromStr;
use tokei::Sort;

#[derive(Clone)]
#[pyclass(name = "Sort")]
pub struct PySort {
    pub sort: Sort,
}

#[pymethods]
impl PySort {
    #[new]
    fn new(s: &str) -> Result<Self, pyo3::PyErr> {
        match Sort::from_str(s) {
            Ok(sort_obj) => Ok(PySort { sort: sort_obj }),
            Err(_) => Err(PyValueError::new_err(format!(
                "Sort type doesn't exists: {}",
                s
            ))),
        }
    }

    #[staticmethod]
    pub fn from_str(s: &str) -> Result<Self, pyo3::PyErr> {
        Self::new(s)
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Sort({:#?})", self.sort))
    }
}

#[pyfunction]
pub fn sort_types() -> Vec<String> {
    let sort_types = Vec::from([
        "Blanks".to_string(),
        "Comments".to_string(),
        "Code".to_string(),
        "Files".to_string(),
        "Lines".to_string(),
    ]);
    sort_types
}
