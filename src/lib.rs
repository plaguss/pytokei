/*
use std::{
    collections::{btree_map, BTreeMap},
    iter::IntoIterator,
    ops::{AddAssign, Deref, DerefMut},
    path::Path,
};
*/

use pyo3::prelude::*;
use tokei::{
    Config,
    Languages,
    Sort,
//    LanguageType
};
use std::collections::HashMap;
use std::str::FromStr;
//use std::path::Path;
use pyo3::types::{
//    PyList,
    PyString,
    PyDict
};
//use pyo3::types::PyType;

#[pyclass]
pub struct PySort {
    pub sort: Sort
}


#[pymethods]
impl PySort {
    #[new]
    fn py_new() -> Self {
        PySort{sort: Sort::Lines}
    }

    #[staticmethod]
    pub fn from_str(s: &str) -> PyResult<PySort> {
        // NOTE: Take care of the error of 
        Ok(PySort{sort: Sort::from_str(s).unwrap()})
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("PySort({:#?})", self.sort))
    }
}


#[pyclass]
pub struct PyConfig {
    pub config: Config
}


#[pymethods]
impl PyConfig {
    #[new]
    fn py_new() -> Self {
        PyConfig{config: Config::default()}
    }

    #[getter]
    fn columns(&self) -> Option<usize> {
        self.config.columns
    }

    #[getter]
    fn hidden(&self) -> Option<bool> {
        self.config.hidden
    }

    //
    #[getter]
    fn no_ignore(&self) -> Option<bool> {
        self.config.no_ignore
    }

    #[getter]
    fn no_ignore_parent(&self) -> Option<bool> {
        self.config.no_ignore_parent
    }

    #[getter]
    fn no_ignore_dot(&self) -> Option<bool> {
        self.config.no_ignore_dot
    }

    #[getter]
    fn no_ignore_vcs(&self) -> Option<bool> {
        self.config.no_ignore_vcs
    }

    #[getter]
    fn treat_doc_strings_as_comments(&self) -> Option<bool> {
        self.config.treat_doc_strings_as_comments
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
}


#[pyclass]
pub struct PyLanguages {
    pub languages: Languages
}

#[pymethods]
impl PyLanguages {
    #[new]
    fn py_new() -> Self {
        PyLanguages{languages: Languages::new()}
    }

    pub fn get_statistics(
        &mut self,
        paths: &PyString,
        ignored: &PyString,
        config: &PyConfig
    ) {
        // NOTE: First attempt using only a string
        let binding_path = paths.to_string();
        let paths_: &str = binding_path.as_ref();

        let ignored_path = ignored.to_string();
        let ignored_: &str = ignored_path.as_ref();

        self.languages.get_statistics(&[paths_], &[&ignored_], &config.config)
    }

    pub fn summary(&self) -> PyResult<PyObject> {
    // NOTE: General form of returning a dict from a HasMap.
    // NOTE: Now we need a way to obtain this HashMap
    // ref: https://py4u.org/questions/70193869/
//    pub fn summary(&self) -> PyResult<PyDict> {
        let mut map = HashMap::new();
        map.insert("Python", "summary");
        return pyo3::Python::with_gil( |py| {
            Ok( map.to_object( py ) )
        } );
    }

}

/*
#[pyclass]
pub struct PyLanguageType {
    #[new]
    fn py_new() -> Self {
        pub type: LanguageType
    }
}
*/
// m.add_class::<PySort>().unwrap();



#[pymodule]
fn _pytokei(_py: Python, m: &PyModule) -> PyResult<()> {
//    m.add_class(wrap_pyfunction!(guess_the_number, m)?)?;

    m.add_class::<PyConfig>().unwrap();
    m.add_class::<PyLanguages>().unwrap();
//    m.add_class::<PyLanguageType>().unwrap();
    m.add_class::<PySort>().unwrap();
    Ok(())

}
