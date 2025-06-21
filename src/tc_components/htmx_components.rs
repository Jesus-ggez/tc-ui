use pyo3::prelude::*;

use crate::dom_components;

//<·
#[pymethods]
pub fn ajax() {
    let _ = dom_components::HtmlElement::new();
}
