use pyo3::prelude::*;

use crate::dom_components::HtmlElement;

//<·
#[pymethods]
impl HtmlElement {
    pub fn tc_prop(slf: PyRefMut<'_, Self>, name: String, value: String) -> PyRefMut<'_, Self> {
        let set_attr_name = format!("data-tc-{}", name);
        Self::set_attr(slf, set_attr_name.to_string(), value)
    }
}

