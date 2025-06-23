use pyo3::prelude::*;

use crate::dom_components::HtmlElement;

//<·
#[pymethods]
impl HtmlElement {
    pub fn set_attr(mut slf: PyRefMut<'_, Self>, name: String, value: String) -> PyRefMut<'_, Self> {
        let _ = slf.attrs.insert(name, value);
        slf
    }
    fn set_attr_controls(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "controls".to_string(), value)
    }
    fn set_attr_loop(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "loop".to_string(), value)
    }
    fn set_attr_type(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "type".to_string(), value)
    }
    fn set_attr_value(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "value".to_string(), value)
    }
    fn set_attr_id(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "id".to_string(), value)
    }
    fn set_attr_class(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "class".to_string(), value)
    }
    fn set_attr_content(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "content".to_string(), value)
    }
}
