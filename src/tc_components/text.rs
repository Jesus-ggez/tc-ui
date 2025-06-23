use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::dom_components::HtmlElement;

//<·
#[pyclass(extends=HtmlElement, subclass)]
pub struct Text {
    #[pyo3(get, set)]
    value: String,
}

#[pymethods]
impl Text {
    #[new]
    #[pyo3(signature=(value = None, tag = None, **_py_kwargs))]
    fn new(
        value: Option<String>,
        tag: Option<String>,
        _py_kwargs: Option<&Bound<'_, PyDict>>,
    ) -> (Self, HtmlElement) {
        let val = match value {
            None => "".into(),
            Some(s) => s,
        };

        let mut tag_element = HtmlElement::new(tag.unwrap_or("p".into()), None);
        tag_element.content.push(val.clone());

        (Text { value: val }, tag_element)
    }

    #[pyo3(signature=(value = None, tag=None))]
    fn __init__(mut slf: PyRefMut<'_, Self>, value: Option<String>, tag: Option<String>) {
        if value.is_some() {
            slf.value = value.unwrap();
        }

        if tag.is_some() {
            slf.as_super().tag = tag.unwrap()
        }

        slf.as_super().components = vec![slf.value.clone()]
    }

    fn __str__(slf: PyRef<'_, Self>) -> PyResult<String> {
        slf.as_super().__str__()
    }

    #[setter]
    fn set_value(mut slf: PyRefMut<'_, Self>, value: String) {
        slf.value = value;
        slf.as_super().components = vec![slf.value.clone()];
        slf.as_super().__update_content();
    }
}
