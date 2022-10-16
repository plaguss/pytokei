use std::collections::HashMap;

use pyo3::prelude::*;

use tokei::Language;

use crate::pylanguage_type::PyLanguageType;
use crate::pysort::PySort;
use crate::pystats::PyReport;

#[pyclass(name = "Language")]
pub struct PyLanguage {
    pub language: Language,
}

#[pymethods]
impl PyLanguage {
    // mark_innacurate is not implemented.
    // https://docs.rs/tokei/latest/tokei/struct.Language.html#impl
    #[new]
    pub fn new() -> Self {
        PyLanguage {
            language: Language::new(),
        }
    }

    #[getter]
    pub fn blanks(&self) -> usize {
        self.language.blanks
    }

    #[getter]
    pub fn code(&self) -> usize {
        self.language.code
    }

    #[getter]
    pub fn comments(&self) -> usize {
        self.language.comments
    }

    #[getter]
    pub fn reports(&self) -> Vec<PyReport> {
        // Check for implementation of map on the iterator:
        // https://stackoverflow.com/questions/30026893/using-map-with-vectors
        let mut reports = Vec::new();
        let inner = self.language.reports.clone();
        for r in &inner {
            reports.push(PyReport { report: r.clone() });
        }
        reports
    }

    #[getter]
    pub fn children(&self) -> HashMap<PyLanguageType, Vec<PyReport>> {
        // This version returns a representation of the internal structure
        let children_ = self.language.children.clone();
        let mut children = HashMap::new();
        for (lang_type, reports) in children_.iter() {
            let mut pyreports = Vec::new();
            for r in reports.iter() {
                pyreports.push(PyReport { report: r.clone() });
            }
            children.insert(PyLanguageType(lang_type.clone()), pyreports);
        }
        children
    }

//    pub fn children_plain(&self) {}

    #[getter]
    pub fn inaccurate(&self) -> bool {
        self.language.inaccurate
    }

    pub fn lines(&self) -> usize {
        self.language.lines()
    }

    pub fn add_report(&mut self, report: PyReport) {
        self.language.add_report(report.report);
    }

    pub fn summarise(&self) -> PyLanguage {
        PyLanguage {
            language: self.language.summarise(),
        }
    }

    pub fn total(&mut self) {
        self.language.total();
    }

    pub fn is_empty(&self) -> bool {
        self.language.is_empty()
    }

    pub fn sort_by(&mut self, category: PySort) {
        self.language.sort_by(category.sort);
    }
}
