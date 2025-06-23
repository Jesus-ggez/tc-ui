use pyo3::prelude::*;

use crate::dom_components::HtmlElement;

//<·
#[pymethods]
impl HtmlElement {
    fn poster(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "poster".to_string(), value)
    }
    fn charset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "charset".to_string(), value)
    }
    fn name(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "name".to_string(), value)
    }
    fn rel(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "rel".to_string(), value)
    }
    fn autoplay(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "autoplay".to_string(), value)
    }
    fn placeholder(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "placeholder".to_string(), value)
    }
    fn required(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "required".to_string(), value)
    }
    fn disabled(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "disabled".to_string(), value)
    }
    fn readonly(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "readonly".to_string(), value)
    }
    fn min(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "min".to_string(), value)
    }
    fn max(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "max".to_string(), value)
    }
    fn step(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "step".to_string(), value)
    }
    fn checked(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "checked".to_string(), value)
    }
    fn style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "style".to_string(), value)
    }
    fn title(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "title".to_string(), value)
    }
    fn lang(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "lang".to_string(), value)
    }
    fn tabindex(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "tabindex".to_string(), value)
    }
    fn contenteditable(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "contenteditable".to_string(), value)
    }
    fn hidden(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "hidden".to_string(), value)
    }
    fn draggable(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "draggable".to_string(), value)
    }
    fn href(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "href".to_string(), value)
    }
    fn src(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "src".to_string(), value)
    }
    fn alt(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "alt".to_string(), value)
    }
    fn target(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "target".to_string(), value)
    }
    fn height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "height".to_string(), value)
    }
}
