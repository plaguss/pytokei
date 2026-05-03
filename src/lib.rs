use pyo3::prelude::*;

use crate::pyconfig::PyConfig;
use crate::pylanguage::PyLanguage;
use crate::pylanguage_type::PyLanguageType;
use crate::pylanguages::PyLanguages;
use crate::pysort::{sort_types, PySort};
use crate::pystats::{PyCodeStats, PyReport};

pub mod pyconfig;
pub mod pylanguage;
pub mod pylanguage_type;
pub mod pylanguages;
pub mod pysort;
pub mod pystats;

#[pymodule]
fn _pytokei(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let version = env!("CARGO_PKG_VERSION").to_string();
    m.add("__version__", version)?;
    m.add_class::<PyConfig>()?;
    m.add_class::<PyLanguages>()?;
    m.add_class::<PySort>()?;
    m.add_function(wrap_pyfunction!(sort_types, m)?)?;
    m.add_class::<PyCodeStats>()?;
    m.add_class::<PyReport>()?;
    m.add_class::<PyLanguageType>()?;
    m.add_class::<PyLanguage>()?;
    Ok(())
}
