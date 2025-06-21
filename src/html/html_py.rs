use pyo3::prelude::*;
use std::collections::HashMap;

use crate::dom_components::HtmlElement;

//<·
#[pymethods]
impl HtmlElement {
    #[new]
    #[pyo3(signature=(tag = "div".to_string()))]
    pub fn new(tag: String) -> Self {
        HtmlElement {
            tag: tag,
            attrs: HashMap::new(),
            components: Vec::new(),
        }
    }

    fn __init__(&self, tag: String) {
        self.tag = tag;
    }

    pub fn __str__(&self) -> PyResult<String> {
        let mut attrs: String = self.__list_fmt_attrs().join("");
        if !attrs.is_empty() {
            attrs = " ".to_owned() + &attrs;
        }

        Ok(format!(
            "<{}{}>{}</{}>",
            &self.tag,
            attrs,
            self.__list_clear_components().join(""),
            &self.tag
        ))
    }
}
