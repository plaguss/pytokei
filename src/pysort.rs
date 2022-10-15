use pyo3::prelude::*;
use std::str::FromStr;
use tokei::Sort;

#[pyclass(name = "Sort")]
pub struct PySort {
    pub sort: Sort,
}

#[pymethods]
impl PySort {
    #[new]
    fn py_new() -> Self {
        PySort { sort: Sort::Lines }
    }

    #[staticmethod]
    pub fn from_str(s: &str) -> PyResult<PySort> {
        // NOTE: Take care of the error of
        Ok(PySort {
            sort: Sort::from_str(s).unwrap(),
        })
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Sort({:#?})", self.sort))
    }
}

/*
NOT DEVELOPED YET
    #[getter]
    fn sort(&self) -> Option<PySort> {
//        Ok(PySort(sort: self.config.sort))
        self.config.sort
    }

    #[getter]
    fn types(&self) -> Option<Vec<LanguageType>> {
    // NOTE: RETURN Option<Vec<String>> TRANSFORMING TO THE INNER LanguageType.name()
//        Ok(PySort(sort: self.config.sort))
        self.config.types
    }

*/
