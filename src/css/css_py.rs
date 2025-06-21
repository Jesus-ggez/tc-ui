use pyo3::prelude::*;
use std::collections::HashMap;

use crate::dom_components::StyleComponent;

//<·
#[pymethods]
impl StyleComponent {
    #[new]
    fn new() -> Self {
        StyleComponent {
            properties: HashMap::new(),
        }
    }

    fn __init__(&mut self, properties: HashMap<String, String>) {
        self.properties = properties;
    }

    pub fn __str__(&self) -> PyResult<String> {
        let fmt: String = self
            .__simple_base_content()
            .iter()
            .map(|entry| format!("\t{}\n", entry))
            .collect();

        Ok(format!("{{\n{}\n}}", fmt))
    }
}
