use pyo3::prelude::*;

use crate::dom_components::HtmlElement;

//<·
#[pymethods]
impl HtmlElement {
    #[setter]
    pub fn set_content(&mut self, ctt: Vec<HtmlElement>) {
        self.html_content.clear();
        self.widgets.clear();
        self.content = ctt.iter()
            .map(|h| h.as_tag().unwrap())
            .collect();
    }

    #[getter]
    pub fn get_content(&self) -> PyResult<Vec<String>> {
        Ok(self.content.clone())
    }

    pub fn append_html(&mut self, item: String) {
        self.html_content.push(item.clone());
        self.content.push(item);
    }

    pub fn append(&mut self, item: HtmlElement) {
        self.widgets.push(item.clone());
        self.content.push(item.as_tag().unwrap());
    }

    pub fn extend_html(&mut self, ctt: Vec<String>) {
        self.content = vec![]
            .into_iter()
            .chain(self.content.clone())
            .chain(ctt)
            .collect()

    }
    pub fn extend(&mut self, ctt: Vec<HtmlElement>) {
        let content: Vec<String> = ctt.iter()
            .map(|c| c.as_tag().unwrap())
            .collect();

        self.content = vec![]
            .into_iter()
            .chain(self.content.clone())
            .chain(content)
            .collect()
    }
}

