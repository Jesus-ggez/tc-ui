use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

use crate::dom_components::HtmlElement;

//<·
#[pymethods]
impl HtmlElement {
    #[new]
    #[pyo3(signature=(tag = "div".to_string(), **py_kwargs))]
    pub fn new(tag: String, py_kwargs: Option<&Bound<'_, PyDict>>) -> Self {
        let _ = py_kwargs;
        HtmlElement {
            tag: tag,
            attrs: HashMap::new(),
            components: Vec::new(),
            composition: Vec::new(),
        }
    }

    #[pyo3(signature=(tag = None))]
    fn __init__(&mut self, tag: Option<String>) {
        if let Some(t) = tag {
            if !t.is_empty() {
                self.tag = t;
            }
        }
    }
    pub fn __str__(&self) -> PyResult<String> {
        let components: String = self.__list_trimed_components().join("");
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
