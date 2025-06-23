use pyo3::prelude::*;
use std::collections::HashMap;

//<·
///stub_gen only read
/// <· type ·> == -> type
/// #.? -> ignore

#[pyclass(subclass)]
pub struct StyleComponent {
    #[pyo3(get, set)]
    pub properties: HashMap<String, String>,
}

#[pyclass(subclass)]
#[derive(Debug, Clone)]
pub struct HtmlElement {
    #[pyo3(get)]
    pub attrs: HashMap<String, String>,

    #[pyo3(get, set)]
    pub tag: String,

    // cls.append -> cls
    pub widgets: Vec<HtmlElement>,

    // cls.append_html -> str
    pub html_content: Vec<String>,

    /// getter, setter
    /// self.content = [] -> []
    /// # py
    /// html: HtmlElement = HtmlElement('foo')
    /// html.content = [...] -> [...]
    /// print(html.content) -> [...]
    pub content: Vec<String>,
}

