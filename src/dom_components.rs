use pyo3::prelude::*;
use std::collections::HashMap;

//<·
#[pyclass(subclass)]
pub struct StyleComponent {
    #[pyo3(get)]
    pub properties: HashMap<String, String>,
}
/// stub_gen only read
/// <· type ·> == -> type
/// #.? -> ignore

#[pyclass(subclass)]
pub struct HtmlElement {
    #[pyo3(get)]
    pub attrs: HashMap<String, String>,

    #[pyo3(get, set)]
    pub components: Vec<String>,

    #[pyo3(get, set)]
    pub tag: String,
}
