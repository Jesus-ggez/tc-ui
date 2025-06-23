use pyo3::prelude::*;

use crate::dom_components::HtmlElement;

//<·
#[pymethods]
impl HtmlElement {
    #[setter]
    pub fn set_content(&mut self, ctt: Vec<HtmlElement>) {
        self.widgets = ctt;
        self.content.clear();
        self.__update_content();
    }

    #[getter]
    pub fn get_content(&self) -> PyResult<Vec<String>> {
        Ok(self.content.clone())
    }

    pub fn append_html(&mut self, item: String) {
        self.html_content.push(item);
        self.__update_content();
    }

    pub fn append(&mut self, item: HtmlElement) {
        self.widgets.push(item.clone());
        self.__update_content();
    }

    pub fn __update_content(&mut self) {
        let new_content: Vec<String> = vec![]
            .into_iter()
            .chain(self.html_content.clone())
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
