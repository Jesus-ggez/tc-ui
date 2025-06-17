use pyo3::prelude::*;
use std::collections::HashMap;

//<·
#[pyclass]
pub struct StyleComponent {
    #[pyo3(get)]
    pub properties: HashMap<String, String>,
}

#[pyclass]
pub struct HtmlElement{
    #[pyo3(get)]
    pub properties: HashMap<String, String>,

    #[pyo3(get)]
    pub semantic_tag: String,
}
