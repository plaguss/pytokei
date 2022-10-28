use std::collections::HashMap;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use tokei::Languages;

use crate::pyconfig::PyConfig;
use crate::pylanguage::{PyLanguage, ReportsPlain};
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

    pub fn language_names(&self) -> PyResult<Vec<&str>> {
        let vec = self
            .languages
            .iter()
            .map(|(lang_type, _)| lang_type.name())
            .collect();
        Ok(vec)
    }

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

    pub fn files(&self) -> HashMap<&str, usize> {
        let files = self
            .languages
            .iter()
            .map(|(lang_type, lang)| (lang_type.name(), lang.reports.len()))
            .collect();
        files
    }

    pub fn __repr__(&self) -> &str {
        return "Languages()";
    }

    pub fn get_languages_plain(&self) -> HashMap<&str, ReportsPlain> {
        // Corresponds to calling to the general command with --files and --compact
        let map: HashMap<&str, ReportsPlain> = self
            .languages
            .iter()
            .map(|(lang_type, lang)| {
                (
                    lang_type.name(),
                    PyLanguage {
                        language: lang.clone(),
                    }
                    .reports_plain(),
                )
            })
            .collect();
        map
    }

    pub fn total_plain(&self) -> HashMap<&str, usize> {
        // Returns the Total aggregation.
        let lang_total = self.languages.total();
        let map = HashMap::from([
            (
                "files",
                lang_total.children.values().map(Vec::len).sum::<usize>(),
            ),
            ("lines", lang_total.lines()),
            ("code", lang_total.code),
            ("comments", lang_total.comments),
            ("blanks", lang_total.blanks),
        ]);
        map
    }

    pub fn report_compact_plain(&self) -> HashMap<&str, HashMap<&str, usize>> {
        // Returns the info obtained from the default CLI command in compact mode
        let mut report = HashMap::new();

        for (ltype, lang) in &self.languages {
            let mut stats = HashMap::new();
            for r in &lang.reports {
                stats.insert("lines", r.stats.lines());
                stats.insert("code", r.stats.code);
                stats.insert("comments", r.stats.comments);
                stats.insert("blanks", r.stats.blanks);
            }
            stats.insert("files", lang.reports.len());
            report.insert(ltype.name(), stats);
        }
        report
    }
}
