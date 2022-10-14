use std::collections::HashMap;
use std::path::PathBuf;
use pyo3::prelude::*;
use tokei::{
    CodeStats,
    Report
};

#[derive(Clone)]
#[pyclass]
pub struct PyCodeStats {
    pub stats: CodeStats
}

#[pymethods]
impl PyCodeStats {
    #[new]
    pub fn py_new() -> Self {
        PyCodeStats{stats: CodeStats::new()}
    }

    #[getter]
    pub fn blanks(&self) -> usize {
        self.stats.blanks
    }

    #[getter]
    pub fn code(&self) -> usize {
        self.stats.code
    }

    #[getter]
    pub fn comments(&self) -> usize {
        self.stats.comments
    }

    pub fn lines(&self) -> usize {
        self.stats.lines()
    }

    pub fn summarise(&self) -> PyCodeStats {
        // The new object is created by copying and inserting the new summarised values
        let mut new_stats = self.clone();
        let summ = self.stats.summarise();
        new_stats.stats.blanks = summ.blanks;
        new_stats.stats.code = summ.code;
        new_stats.stats.comments = summ.comments;
        return new_stats
    }

    pub fn content(&self) -> PyResult<PyObject> {
        // Obtain the inner content as a dict in Python.
        let mut map = HashMap::new();
        map.insert("blanks", self.blanks());
        map.insert("code", self.code());
        map.insert("comments", self.comments());
        map.insert("lines", self.lines());
        return pyo3::Python::with_gil( |py| {
            Ok( map.to_object( py ) )
        } );
    }

    // fn __repr__(&self) -> PyString {}  // TBD
}


#[pyclass(name="Report")]
pub struct PyReport {
    pub report: Report
}


#[pymethods]
impl PyReport {
    #[new]
    pub fn py_new(name: &str) -> Self {
        let mut path = PathBuf::new();
        path.push(name);
        PyReport{report: Report::new(path)}
    }
}