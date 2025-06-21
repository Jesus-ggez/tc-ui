use pyo3::prelude::*;

use crate::dom_components::HtmlElement;

//<·
#[pymethods]
impl HtmlElement {
    pub fn formated(&self) -> PyResult<String> {
        let mut attrs: String = self
            .__list_fmt_attrs()
            .iter()
            .map(|c| format!("\t{}\n", c))
            .collect();

        let mut controls = self.__list_fmt_components().join("\n");

        if !controls.is_empty() {
            controls = "\n".to_owned() + &controls;
        }

        if !attrs.is_empty() {
            attrs = "\n".to_owned() + &attrs;
        }

        Ok(format!(
            "<{}{}>{}\n</{}>",
            &self.tag, attrs, controls, &self.tag,
        ))
    }

    pub fn as_tag(&self) -> PyResult<String> {
        Ok(self.__str__()?.to_owned() + "\n")
    }
    pub fn append(&mut self, item: String) {
        self.components.push(item)
    }
}
