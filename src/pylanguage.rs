//use std::collections::{HashMap, HashSet};

use pyo3::prelude::*;
//use pyo3::types::PyString;

use tokei::Language;

//use crate::pyconfig::PyConfig;
//use crate::pylanguage_type::LanguageTypeContainer;
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

    /*
    #[getter]
    pub fn children(&self) -> BtreeMap<, Vec<PyReport>> {
        let children = self.language.children;
    }
    */

    #[getter]
    pub fn innacurate(&self) -> bool {
        self.language.inaccurate
    }
}
