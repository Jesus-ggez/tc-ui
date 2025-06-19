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
            components: Vec::new(),
        }
    } // __init__(self)

    fn __str__(&self) -> PyResult<String> {
        let mut entries = Vec::new();

        for (k, v) in &self.properties {
            entries.push(format!("{}=\"{}\"", k, v));
        }
        let self_str = format!(
            "<{} {}>{}</{}>\n",
            &self.repr_tag,
            entries.join(""),
            &self.components.join(""),
            &self.repr_tag,
        );

        Ok(self_str)
    } // __str__

    fn set_attr(mut slf: PyRefMut<'_, Self>, name: String, value: String) -> PyRefMut<'_, Self> {
        let _ = slf.properties.insert(name, value);
        slf
    }
    fn tc_prop(slf: PyRefMut<'_, Self>, name: String, value: String) -> PyRefMut<'_, Self> {
        let set_attr_name = format!("data-tc-{}", name);
        Self::set_attr(slf, set_attr_name.to_string(), value)
    }

    fn set_id(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "id".to_string(), value)
    }
    fn set_class(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "class".to_string(), value)
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
    fn set_type(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "type".to_string(), value)
    }

    fn set_value(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "value".to_string(), value)
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
    fn controls(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "controls".to_string(), value)
    }
    fn autoplay(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "autoplay".to_string(), value)
    }
    fn set_loop(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "loop".to_string(), value)
    }
    fn poster(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "poster".to_string(), value)
    }
    fn charset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "charset".to_string(), value)
    }
    fn name(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "name".to_string(), value)
    }
    fn content(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "content".to_string(), value)
    }
    fn rel(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_attr(slf, "rel".to_string(), value)
    }
}
