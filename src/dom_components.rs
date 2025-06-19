use pyo3::prelude::*;
use std::collections::HashMap;

//<·
#[pyclass(subclass)]
pub struct StyleComponent {
    #[pyo3(get)]
    pub properties: HashMap<String, String>,
}

#[pyclass(subclass)]
pub struct HtmlElement {
    #[pyo3(get)]
    pub properties: HashMap<String, String>,

    #[pyo3(get, set)]
    pub repr_tag: String,

    #[pyo3(get, set)]
    pub components: Vec<String>,
}
