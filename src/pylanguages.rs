use std::collections::{HashMap, HashSet};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

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
    #[new]
    pub fn new() -> Self {
        PyLanguages {
            languages: Languages::new(),
        }
    }

    pub fn get_statistics(&mut self, paths: Vec<String>, ignored: Vec<String>, config: &PyConfig) {
        let paths_: Vec<&str> = paths.iter().map(String::as_str).collect();
        let paths_ = paths_.as_slice();

        let ignored_: Vec<&str> = ignored.iter().map(String::as_str).collect();
        let ignored_ = ignored_.as_slice();

        self.languages
            .get_statistics(&paths_, &ignored_, &config.config)
    }

    pub fn total(&self) -> PyLanguage {
        PyLanguage {
            language: self.languages.total(),
        }
    }

    // Return the set of languages.
    // TODO: Simplify this function
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
            .map(|(x, y)| {
                (
                    PyLanguageType(x.clone()),
                    PyLanguage {
                        language: y.clone(),
                    },
                )
            })
            .collect();
        map
    }

    // Equivalent to get_languages but returns the objects in plain python objects
    // instead of wrapped in classes
    // TODO: First treat the other classes to simplify getting them here.
    /*
    pub fn get_languages_plain(&self) -> HashMap<&'static str, &'static str> {
        let map: HashMap<PyLanguageType, PyLanguage> = self
            .languages
            .iter()
            .map(|(x, y)| (PyLanguageType(x.clone()), PyLanguage { language: y.clone() }))
            .collect();
        map

    }
    */
}
