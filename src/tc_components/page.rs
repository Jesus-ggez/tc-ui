use pyo3::prelude::*;

use crate::dom_components::HtmlElement;

//<·
#[pyclass(extends=HtmlElement, subclass)]
pub struct Page {
    #[pyo3(get, set)]
    require_divider: bool,
}

#[pymethods]
impl Page {
    #[new]
    fn new() -> (Self, HtmlElement) {
        let slf_content = Page {
            require_divider: false,
        };
        let init_tag = "html".to_string();
        let supr_content = HtmlElement::new(init_tag, None);

        (slf_content, supr_content)
    }
    #[pyo3(signature=(value = None, tag = None))]
    fn __init__(_slf: PyRefMut<'_, Self>, value: Option<String>, tag: Option<String>) {
        let _ = value;
        let _ = tag;
    }

    fn formated(&self) {}

    fn as_tag(&self) {}
}
