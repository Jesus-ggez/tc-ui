use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
pub struct StaticComponent {
    #[pyo3(get)]
    pub properties: HashMap<String, String>,
}
