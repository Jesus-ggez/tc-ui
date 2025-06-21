use crate::dom_components::HtmlElement;

//<·
impl HtmlElement {
    pub fn __list_fmt_attrs(&self) -> Vec<String> {
        let mut attrs = Vec::new();

        for (k, v) in &self.attrs {
            attrs.push(format!("{}=\"{}\" ", k, v));
        }
        attrs
    }
    pub fn __list_fmt_components(&self) -> Vec<String> {
        self.components
            .iter()
            .map(|c| format!("\t{}", c.trim()))
            .collect()
    }
    pub fn __list_clear_components(&self) -> Vec<String> {
        self.components.iter().map(|c| c.trim().to_string()).collect()
    }
}
