use pyo3::prelude::*;

use crate::critical::hierarchy::Hierarchy;
use crate::dom_components::HtmlElement;

//<·
#[pymethods]
impl HtmlElement {
    pub fn formated(&self) -> PyResult<String> {
        Ok(self.decompose().join("\n"))
    }

    pub fn as_tag(&self) -> PyResult<String> {
        Ok(self.decompose().join("\n"))
    }

    pub fn add_html(&mut self, item: String) {
        self.components.push(item);
        self.__update_content();
    }

    pub fn append(&mut self, item: HtmlElement) {
        self.widgets.push(item.clone());
        self.__update_content();
    }

    pub fn __update_content(&mut self) {
        let new_content: Vec<String> = vec![]
            .into_iter()
            .chain(self.components.clone())
            .chain(
                self.widgets
                    .iter()
                    .map(|w| w.as_tag().unwrap())
                    .collect::<Vec<String>>(),
            )
            .collect();

        self.content = new_content;
    }
}
