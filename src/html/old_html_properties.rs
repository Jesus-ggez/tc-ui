use pyo3::prelude::*;
use std::collections::HashMap;

use crate::dom_components::HtmlElement;

//<·
#[pymethods]
impl HtmlElement {
    #[new]
    #[pyo3(signature=(tag = "div".to_string()))]
    pub fn new(tag: String) -> Self {
        HtmlElement {
            tag: tag,
            attrs: HashMap::new(),
            components: Vec::new(),
        }
    } // __init__(self)

    fn __str__(&self) -> PyResult<String> { // #.?
        let controls: String = self.components.iter()
            .map(|c| c.trim())
            .collect();

        let mut attrs: String = self.__simple_base_content_attrs().join("");

        if !attrs.is_empty() {
            attrs = " ".to_owned() + &attrs;
        }

        let __str__= format!(
            "<{}{}>{}</{}>",
            &self.tag,
            attrs,
            controls,
            &self.tag,
        );
        Ok(__str__)
    } // __str__

    pub fn formated(&self) -> PyResult<String> { // <·str·>!
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

    pub fn as_tag(&self) -> PyResult<String> { // <·str·>!
        let str_repr = &self.__str__()?;
        Ok(str_repr.to_owned() + "\n")
    } // __as_tag__

    /// html attrs
}
