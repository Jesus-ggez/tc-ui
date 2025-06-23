use pyo3::prelude::*;

use crate::critical::hierarchy::Hierarchy;
use crate::dom_components::HtmlElement;

//<·
#[pymethods]
impl HtmlElement {
    pub fn formated(&self) -> PyResult<String> {
        Ok(self.decompose().join("\n"))
    }

    pub fn as_tag(&self) -> PyResult<String> {
        Ok(self.decompose().join("\n"))
    }
}
