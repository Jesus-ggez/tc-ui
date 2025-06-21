use crate::dom_components::StyleComponent;

//<·
impl StyleComponent {
    pub fn __simple_base_content(&self) -> Vec<String> {
        let mut entries = Vec::new();

        for (k, v) in &self.properties {
            entries.push(format!("{}: {};", k, v));
        }
        entries
    }
}
