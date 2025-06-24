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
    #[pyo3( signature = (value = None, tag = None, **_py_kwargs))]
    fn new(
        value: Option<String>,
        tag: Option<String>,
        _py_kwargs: Option<&Bound<'_, PyDict>>,
    ) -> (Self, HtmlElement) {
        let t = match tag {
            None => "p".into(),
            Some(v) => v,
        };

        let mut tag_element = HtmlElement::new(t, None);

        let val = match value {
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
