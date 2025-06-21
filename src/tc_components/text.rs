use pyo3::prelude::*;

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
    #[pyo3(signature=(value = "".to_string(), semantic_tag=None))]
    fn new(value: String, semantic_tag: Option<String>) -> (Self, HtmlElement) {
        let tag: String = semantic_tag.unwrap_or_else(|| "p".to_string());
        (
            Text{
                value: value,
            },
            HtmlElement::new(tag),
        )
    }
    fn __str__(slf: PyRef<'_, Self>) -> PyResult<String> {
        let supr = slf.as_super();

        supr.__str__()

        /*
        Ok(format!(
            "<{}{}>{}</{}>",
            supr.tag,
            &super_attrs,
            slf.value,
            supr.tag,
        ))
        let attrs = slf.element.__list_fmt_attrs().join("");

        Ok(format!(
            "<{} {}>{}</{}>",
            self.element.tag, attrs, self.value, self.element.tag,
        ))
        */
    }
}
