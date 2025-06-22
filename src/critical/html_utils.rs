use crate::dom_components::HtmlElement;

//<·
impl HtmlElement {
    pub fn __list_attrs(&self) -> Vec<String> {
        let mut attrs = Vec::new();

        for (k, v) in &self.attrs {
            attrs.push(format!("{}=\"{}\"", k, v));
        }
        attrs
    }
    pub fn __list_tab_components(&self) -> Vec<String> {
        self.content
            .iter()
            .map(|c| format!("\t{}", c))
            .collect()
    }
    pub fn __list_trimed_components(&self) -> Vec<String> {
        self.content.iter().map(|c| c.trim().to_string()).collect()
    }
}
