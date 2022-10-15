use pyo3::prelude::*;

use crate::pyconfig::PyConfig;
use crate::pylanguage::PyLanguage;
use crate::pylanguage_type::{language_types, LanguageTypeContainer};
use crate::pylanguages::PyLanguages;
use crate::pysort::PySort;
use crate::pystats::{PyCodeStats, PyReport};

pub mod pyconfig;
pub mod pylanguage;
pub mod pylanguage_type;
pub mod pylanguages;
pub mod pysort;
pub mod pystats;

#[pymodule]
fn _pytokei(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyConfig>().unwrap();
    m.add_class::<PyLanguages>().unwrap();
    m.add_class::<PySort>().unwrap();
    m.add_class::<PyCodeStats>().unwrap();
    m.add_class::<PyReport>().unwrap();
    m.add_function(wrap_pyfunction!(language_types, m)?)?;
    m.add_class::<LanguageTypeContainer>().unwrap();
    m.add_class::<PyLanguage>().unwrap();
    Ok(())
}
