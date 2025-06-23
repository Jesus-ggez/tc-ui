use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

use crate::dom_components::HtmlElement;

//<·
#[pymethods]
impl HtmlElement {
    #[new]
    #[pyo3(signature=(tag = "div".to_string(), *, **_py_kwargs))]
    pub fn new(tag: String, _py_kwargs: Option<&Bound<'_, PyDict>>) -> Self {
        HtmlElement {
            tag: tag,
            attrs: HashMap::new(),
            widgets: Vec::new(),
            html_content: Vec::new(),
            content: Vec::new(),
        }
    }

    #[pyo3(signature=(tag = None, *, **_py_kwargs))]
    fn __init__(&mut self, tag: Option<String>, _py_kwargs: Option<&Bound<'_, PyDict>>) {
        if !tag.is_none() {
            self.tag = tag.unwrap();
        }
    }
    pub fn __str__(&self) -> PyResult<String> {
        let components: String = self.__list_trimed_content().join("");
        let mut attrs: String = self.__list_attrs().join(" ");

        if !attrs.is_empty() {
            attrs = " ".to_owned() + &attrs;
        }

        Ok(format!(
            "<{}{}>{}</{}>",
            &self.tag, attrs, components, &self.tag,
        ))
    }
}
