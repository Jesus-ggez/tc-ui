use pyo3::prelude::*;

use crate::dom_components::HtmlElement;

//<·
#[pyclass(extends=HtmlElement, subclass)]
pub struct Button {
    #[pyo3(get, set)]
    text: String,

    #[pyo3(get, set)]
    on_click: Option<PyObject>,
}

#[pymethods]
impl Button {
    #[new]
    #[pyo3(signature=(text = "".to_string(), on_click = None))]
    fn new(text: String, on_click: Option<PyObject>) -> (Self, HtmlElement) {
        let py_click = Some(on_click).unwrap_or_else(|| None);
        (
            Button {
                on_click: py_click,
                text: text,
            },
            HtmlElement::new("button".to_string(), None),
        )
    }
    fn __str__(slf: PyRef<'_, Self>) -> PyResult<String> {
        let supr = slf.as_super();
        supr.__str__()
    }
}
