use std::collections::HashMap;

use pyo3::prelude::*;

use tokei::Language;

use crate::pylanguage_type::PyLanguageType;
use crate::pysort::PySort;
use crate::pystats::PyReport;

type LanguageChildrenPlain = HashMap<String, Vec<HashMap<String, HashMap<&'static str, usize>>>>;
type ReportsPlain = Vec<HashMap<String, HashMap<&'static str, usize>>>;

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
        let mut reports = Vec::new();
        let inner = self.language.reports.clone();
        for r in &inner {
            reports.push(PyReport { report: r.clone() });
        }
        reports
    }

    pub fn reports_plain(&self) -> ReportsPlain {
        let mut reports_plain = Vec::new();
        let reports = self.reports();
        for r in &reports {
            reports_plain.push(r.plain());
        }
        reports_plain
    }

    #[getter]
    pub fn children(&self) -> HashMap<PyLanguageType, Vec<PyReport>> {
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

    pub fn children_plain(&self) -> LanguageChildrenPlain {
        let children_ = self.children();
        let mut children_plain = HashMap::new();
        for (py_lang_type, py_reports) in children_.iter() {
            let mut pyreports_plain = Vec::new();
            for r in py_reports.iter() {
                pyreports_plain.push(r.plain());
            }
            children_plain.insert(py_lang_type.name(), pyreports_plain);
        }
        children_plain
    }

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

    pub fn __repr__(&self) -> String {
        format!("Language(empty: {:?})", self.is_empty())
    }

    pub fn files(&self) -> usize {
        self.language.reports.len()
    }
}
