use pyo3::prelude::*;

use crate::dom_components::HtmlElement;
// use crate::utils::str_replace::replace;

//<·
#[pyclass(extends=HtmlElement, subclass)]
pub struct ____ {
    #[pyo3(get, set)]
    value: String,
}

#[pymethods]
impl ____ {
    #[new]
    #[pyo3(signature=(value = None))]
    fn new(value: Option<String>) -> (Self, HtmlElement) {
        let new_value: String = value.unwrap_or_else(|| "".to_string());
        let slf_content = ____ {
            value: new_value.clone(),
        };
        let init_tag = "".to_string();
        let mut supr_content = HtmlElement::new(init_tag, None);

        supr_content.append_html(new_value);
        (slf_content, supr_content)
    }
    #[pyo3(signature=(value = None, tag = None))]
    fn __init__(mut slf: PyRefMut<'_, Self>, value: Option<String>, tag: Option<String>) {
    }

    pub fn __str__(slf: PyRef<'_, Self>) -> PyResult<String> {
    }

    #[setter]
    fn set_value(mut slf: PyRefMut<'_, Self>, value: String) {
        slf.value = value;
        slf.as_super().components = vec![slf.value.clone()];
    }


    fn formated(&self) {}

    fn as_tag(&self) {}
}
