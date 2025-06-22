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
    #[pyo3(signature=(value = None, tag = None, **py_kwargs))]
    fn new(
        value: Option<String>,
        tag: Option<String>,
        py_kwargs: Option<&Bound<'_, PyDict>>,
    ) -> (Self, HtmlElement) {
        let _ = py_kwargs;
        let new_value: String = value.unwrap_or_else(|| "".to_string());
        let new_tag: String = tag.unwrap_or_else(|| "p".to_string());

        let slf_content = Text {
            value: new_value.clone(),
        };
        let mut supr_content = HtmlElement::new(new_tag, None);

        supr_content.append(new_value);
        (slf_content, supr_content)
    }

    #[pyo3(signature=(value = None, tag=None))]
    fn __init__(mut slf: PyRefMut<'_, Self>, value: Option<String>, tag: Option<String>) {
        slf.value = value.unwrap_or_default();
        slf.as_super().components = vec![slf.value.clone()];

        slf.as_super().tag = tag.filter(|t| !t.is_empty()).unwrap_or_default();
    }

    fn __str__(slf: PyRef<'_, Self>) -> PyResult<String> {
        let supr = slf.as_super();
        supr.__str__()
    }

    #[setter]
    fn set_value(mut slf: PyRefMut<'_, Self>, value: String) {
        slf.value = value.clone();
        slf.as_super().components = vec![value];
    }
}
