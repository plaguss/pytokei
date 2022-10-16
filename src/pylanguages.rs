use std::collections::{HashMap, HashSet};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;

use tokei::Languages;

use crate::pyconfig::PyConfig;
use crate::pylanguage::PyLanguage;
use crate::pylanguage_type::PyLanguageType;

#[pyclass(name = "Languages")]
pub struct PyLanguages {
    pub languages: Languages,
}

#[pymethods]
impl PyLanguages {
    // Define a method __getattr__ to retrieve the languages:
    // https://docs.rs/tokei/latest/tokei/
    #[new]
    pub fn new() -> Self {
        PyLanguages {
            languages: Languages::new(),
        }
    }

    pub fn get_statistics(&mut self, paths: &PyString, ignored: &PyString, config: &PyConfig) {
        // NOTE: First attempt using only a string,
        // in tokei it takes a list of paths
        let binding_path = paths.to_string();
        let paths_: &str = binding_path.as_ref();

        let ignored_path = ignored.to_string();
        let ignored_: &str = ignored_path.as_ref();

        self.languages
            .get_statistics(&[paths_], &[&ignored_], &config.config)
    }

    pub fn total(&self) -> PyLanguage {
        PyLanguage {
            language: self.languages.total(),
        }
    }

    // Return the set of languages.
    pub fn language_names(&self) -> PyResult<PyObject> {
        let mut names = HashSet::new();
        //        let keys: Vec<_> = self.languages.keys().cloned().collect();
        // transform the loop to .keys().map() with a closure if possible
        for (lang, _) in &self.languages {
            names.insert(lang.name());
        }
        return pyo3::Python::with_gil(|py| Ok(names.to_object(py)));
    }

    // Implement the same functionality as in the main example.
    // Corresponds to let rust = &languages[&LanguageType::Rust]; in python
    pub fn __getitem__(&self, lang_type: &PyLanguageType) -> Result<PyLanguage, PyErr> {
        let maybe_lang = self.languages.get(&lang_type.0);

        match maybe_lang {
            Some(maybe_lang) => Ok(PyLanguage {
                language: maybe_lang.clone(),
            }),
            None => Err(PyValueError::new_err(format!(
                "LanguageType not found: {}",
                lang_type.0
            ))),
        }
    }

    // Exposes the inner structure with the corresponding python classes
    pub fn get_languages(&self) -> HashMap<PyLanguageType, PyLanguage> {
        let map: HashMap<PyLanguageType, PyLanguage> = self
            .languages
            .iter()
            .map(|(x, y)| (PyLanguageType(x.clone()), PyLanguage { language: y.clone() }))
            .collect();
        map
    }

    /*
    pub fn get_languages_plain(&self) -> {
        /* Equivalent to get_languages but returns the objects in plain python objects
        instead of wrapped in classes
        */
    }
    */
}
