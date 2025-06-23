use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

use crate::dom_components::StyleComponent;

//<·
#[pymethods]
impl StyleComponent {
    #[new]
    #[pyo3(signature=(properties = None, *, **_py_kwargs))]
    fn new(
        properties: Option<HashMap<String, String>>,
        _py_kwargs: Option<&Bound<'_, PyDict>>,
    ) -> Self {
        StyleComponent {
            properties: properties.unwrap_or(HashMap::new()),
        }
    }

    #[pyo3(signature=(properties = None, *, **_py_kwargs))]
    fn __init__(
        &mut self,
        properties: Option<HashMap<String, String>>,
        _py_kwargs: Option<&Bound<'_, PyDict>>,
    ) {
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
