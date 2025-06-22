use pyo3::prelude::*;
use std::collections::HashMap;

//<·
///stub_gen only read
/// <· type ·> == -> type
/// #.? -> ignore

#[pyclass(subclass)]
pub struct StyleComponent {
    #[pyo3(get, set)]
    pub properties: HashMap<String, String>,
}

#[pyclass(subclass)]
#[derive(Debug, Clone)]
pub struct HtmlElement {
    #[pyo3(get)]
    pub attrs: HashMap<String, String>,

    #[pyo3(get, set)]
    pub components: Vec<String>,

    #[pyo3(get, set)]
    pub tag: String,

    #[pyo3(get, set)]
    pub widgets: Vec<HtmlElement>,

    #[pyo3(get, set)]
    pub content: Vec<String>,
}

