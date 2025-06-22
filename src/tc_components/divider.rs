use pyo3::prelude::*;

use crate::dom_components::HtmlElement;

//<·
#[pyclass(extends=HtmlElement, subclass)]
pub struct Divider;

#[pymethods]
impl Divider {
    #[new]
    fn new() -> (Self, HtmlElement) {
        let init_tag = "div".to_string();
        let mut supr_content = HtmlElement::new(init_tag, None);

        supr_content.attrs.insert(
            "class".to_string(),
            "divider".to_string()
        );
        (Divider {}, supr_content)
    }

    #[pyo3(signature=(value = None, tag = None))]
    fn __init__(_slf: PyRefMut<'_, Self>, value: Option<String>, tag: Option<String>) {
        let _ = value;
        let _ = tag;
    }

    fn formated(&self) {}

    fn as_tag(&self) {}
}
