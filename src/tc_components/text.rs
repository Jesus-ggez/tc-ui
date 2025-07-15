use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::dom_components::HtmlElement;

//<·
#[pyclass(extends=HtmlElement, subclass)]
#[derive(Debug)]
pub struct Text {
    #[pyo3(get)]
    value: String,
}

#[pymethods]
impl Text {
    #[new]
    #[pyo3( signature = (value = None, tag = None, **kwargs))]
    fn new(
        value: Option<String>,
        tag: Option<String>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> (Self, HtmlElement) {
        let _ = kwargs;
        let t = match tag {
            None => "p".into(),
            Some(v) => v,
        };

        let mut tag_element: HtmlElement = HtmlElement::new(t, None);

        let val: String = match value {
            None => "".into(),
            Some(s) => {
                tag_element.content.push(s.clone());
                s
            }
        };

        (Text { value: val }, tag_element)
    }

    fn __str__(slf: PyRef<'_, Self>) -> PyResult<String> {
        slf.as_super().as_tag()
    }

    #[setter]
    fn set_value(mut slf: PyRefMut<'_, Self>, value: String) {
        slf.value = value;
        slf.as_super().content = vec![slf.value.clone()];
    }
}
