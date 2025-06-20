use pyo3::prelude::*;
use std::collections::HashMap;

use crate::dom_components::HtmlElement;

//<·
impl HtmlElement {
    fn __simple_base_content_attrs(&self) -> Vec<String> {
        let mut attrs = Vec::new();

        for (k, v) in &self.attrs {
            attrs.push(format!("{}=\"{}\" ", k, v));
        }
        attrs
    }
    fn __simple_base_content_controls(&self) -> Vec<String> {
        self.components.iter()
            .map(|c| format!("\t{}", c.trim()))
            .collect()
    }
}
#[pymethods]
impl HtmlElement {
    #[new]
    #[pyo3(signature=(tag = None))]
    fn new(tag: Option<String>) -> Self {
        let html_tag: String = tag.unwrap_or("div".to_string());

        HtmlElement {
            tag: html_tag,
            attrs: HashMap::new(),
            components: Vec::new(),
        }
    } // __init__(self)

    fn __str__(&self) -> PyResult<String> { // #.?
        let controls: String = self.components.iter()
            .map(|c| c.trim())
            .collect();

        let __str__= format!(
            "<{} {}>{}</{}>",
            &self.tag,
            self.__simple_base_content_attrs().join(""),
            controls,
            &self.tag,
        );
        Ok(__str__)
    } // __str__

    fn formated(&self) -> PyResult<String> { // <·str·>!
        let mut attrs: String = self.__simple_base_content_attrs().iter()
            .map(|c| format!("\t{}\n", c))
            .collect();

        let mut controls = self.__simple_base_content_controls().join("\n");

        if !controls.is_empty() {
            controls = "\n".to_owned() + &controls;
        }
        if !attrs.is_empty() {
            attrs = "\n".to_owned() + &attrs;
        }

        let self_str = format!(
            "<{}{}>{}\n</{}>",
            &self.tag,
            attrs,
            controls,
            &self.tag,
        );

        Ok(self_str)
    } // __formated__

    fn as_tag(&self) -> PyResult<String> { // <·str·>!
        let str_repr = &self.__str__()?;
        Ok(str_repr.to_owned() + "\n")
    } // __as_tag__

    /// html attrs
    fn set_attr(mut slf: PyRefMut<'_, Self>, name: String, value: String) -> PyRefMut<'_, Self> {
        let _ = slf.attrs.insert(name, value);
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
    fn set_controls(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
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
    fn append(&mut self, item: String) {
        let _ = self.components.push(item);
    }
}
