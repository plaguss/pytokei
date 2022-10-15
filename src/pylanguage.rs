use std::collections::HashMap;

use pyo3::prelude::*;

use tokei::Language;

use crate::pylanguage_type::PyLanguageType;
use crate::pystats::PyReport;

#[pyclass(name = "Language")]
pub struct PyLanguage {
    pub language: Language,
}

#[pymethods]
impl PyLanguage {
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
        let mut reports = Vec::new();
        let inner = self.language.reports.clone();
        for r in &inner {
            reports.push(PyReport{report: r.clone()});
        }
        reports
    }
    
    #[getter]
    pub fn children(&self) -> HashMap<PyLanguageType, Vec<PyReport>> {
        let children_ = self.language.children.clone();
        let mut children = HashMap::new();
        for (lang_type, reports) in children_.iter() {
            let mut pyreports = Vec::new();
            for r in reports.iter() {
                pyreports.push(PyReport{report: r.clone()});
            }
            children.insert(PyLanguageType(lang_type.clone()), pyreports);
        }
        children
    }
    
    #[getter]
    pub fn innacurate(&self) -> bool {
        self.language.inaccurate
    }
}
