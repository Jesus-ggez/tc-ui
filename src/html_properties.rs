use pyo3::prelude::*;
use std::collections::HashMap;

use crate::dom_components::HtmlElement;

//<·

#[pymethods]
impl HtmlElement {
    #[new]
    fn new() -> Self {
        HtmlElement {
            repr_tag: "div".to_string(),
            properties: HashMap::new(),
        }
    }

    fn set_attr(mut slf: PyRefMut<'_, Self>, name: String, value: String) -> PyRefMut<'_, Self> {
        let _ = slf.properties.insert(name, value);
        slf
    }

    /*
    fn set_class(slg: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr()
    }
        // Self::set_property(slf, "align-content".to_string(), value)
    */
    fn set_class(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "class".to_string(), value)
    }

    //__data__
}
