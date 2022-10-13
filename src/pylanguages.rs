use std::collections::{HashMap, HashSet};

use pyo3::prelude::*;
use pyo3::types::PyString;

use tokei::Languages;

use crate::pyconfig::PyConfig;


#[pyclass]
pub struct PyLanguages {
    pub languages: Languages
}

#[pymethods]
impl PyLanguages {
    // Define a method __getattr__ to retrieve the languages:
    // https://docs.rs/tokei/latest/tokei/
    #[new]
    pub fn py_new() -> Self {
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

    // Return the set of languages.
    pub fn language_names(&self) -> PyResult<PyObject> {
        let mut names = HashSet::new();
//        let keys: Vec<_> = self.languages.keys().cloned().collect();
        for (lang, _) in &self.languages {
            names.insert(lang.name());
        }
        return pyo3::Python::with_gil( |py| {
            Ok(names.to_object(py))
        } );
    }

}
