use pyo3::prelude::*;

use crate::critical::hierarchy::Hierarchy;
use crate::dom_components::{HtmlElement, StyleComponent};

//<·
#[pymethods]
impl StyleComponent {
    fn as_class(&self, class_name: String) -> PyResult<String> {
        let mut css_content = self.composition();
        if css_content.is_empty() {
            return Ok(format!("{} {{}}", class_name));
        }

        css_content[0] = format!("{} {}", class_name, css_content[0]);

        Ok(css_content.join("\n"))
    }

    fn as_tag(&self, class_name: String) -> PyResult<String> {
        let mut tag_content = self.composition();
        let mut tag = HtmlElement::new("style".to_string(), None);

        if tag_content.is_empty() {
            return tag.__str__();
        }
        tag_content[0] = format!("{} {}", class_name, tag_content[0]);
        tag.content.extend(tag_content);

        Ok(tag.decompose().join("\n"))
    }

    #[pyo3(signature=(use_attr=false))]
    fn as_inline(&self, use_attr: bool) -> PyResult<String> {
        let attrs = self.__list_properties().join(" ");
        if use_attr {
            return Ok(format!("style=\"{}\"", attrs));
        }
        Ok(attrs)
    }
}

