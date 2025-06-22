use pyo3::prelude::*;
use std::collections::HashMap;

//<·
#[pyclass(subclass)]
pub struct StyleComponent {
    #[pyo3(get, set)]
    pub properties: HashMap<String, String>,
}
/// stub_gen only read
/// <· type ·> == -> type
/// #.? -> ignore

//#[pyclass(extends=PyObject, subclass)]
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
    pub composition: Vec<HtmlElement>,
}
