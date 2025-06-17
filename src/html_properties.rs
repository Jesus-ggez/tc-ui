use pyo3::prelude::*;
use std::collections::HashMap;

use crate::dom_components::HtmlElement;

//<·

#[pymethods]
impl HtmlElement {
    #[new]
    fn new() -> Self {
        HtmlElement {
            semantic_tag: "div".to_string(),
            properties: HashMap::new(),
        }
    }
}
