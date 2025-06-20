use pyo3::prelude::*;

use crate::dom_components::HtmlElement;

//<·
#[pyclass]
pub struct Text {
    #[pyo3(get, set)]
    value: String,

    #[pyo3(get, set)]
    pub element: HtmlElement,
}

#[pymethods]
impl Text {
    #[new]
    #[pyo3(signature=(value, semantic_tag=None))]
    fn new(value: String, semantic_tag: Option<String>) -> Self {
        let tag: String = semantic_tag.unwrap_or_else(|| "p".to_string());

        Text {
            value: value,
            element: HtmlElement::new(Some(tag)),
        }
    }
    fn __str__(&self) -> PyResult<String> {
        let attrs = self.element.__simple_base_content_attrs().join("");

        Ok(format!(
            "<{} {}>{}</{}>",
            self.element.tag, attrs, self.value, self.element.tag,
        ))
    }
    fn formated(&mut self) -> PyResult<String> {
        self.element.append(self.value.clone());
        self.element.as_tag()
    }
}
