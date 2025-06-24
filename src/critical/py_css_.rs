use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

use crate::dom_components::StyleComponent;

//<·
#[pymethods]
impl StyleComponent {
    #[new]
    #[pyo3(signature=(properties = None, *, **kwargs))]
    fn new(
        properties: Option<HashMap<String, String>>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> Self {
        let _ = kwargs;
        StyleComponent {
            properties: properties.unwrap_or(HashMap::new()),
        }
    }

    #[pyo3(signature=(properties = None, *, **kwargs))]
    fn __init__(
        &mut self,
        properties: Option<HashMap<String, String>>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) {
        let _ = kwargs;
        if properties.is_some() {
            self.properties = properties.unwrap();
        }
    }

    pub fn __str__(&self) -> PyResult<String> {
        if self.properties.is_empty() {
            return Ok(format!("{{  }}"));
        }
        let fmt: String = self
            .__list_properties()
            .iter()
            .map(|entry| format!("\t{}\n", entry))
            .collect();

        Ok(format!("{{\n{}}}", fmt))
    }
}
