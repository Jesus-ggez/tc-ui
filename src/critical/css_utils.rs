use crate::dom_components::StyleComponent;

//<·
impl StyleComponent {
    pub fn __simple_base_content(&self) -> Vec<String> {
        let mut css = Vec::new();

        for (k, v) in &self.properties {
            css.push(format!("{}: {};", k, v));
        }
        css
    }
    pub fn __indent_content(&self) -> Vec<String> {
        self
            .__simple_base_content()
            .iter()
            .map(|line| "\t".to_owned() + line)
            .collect()
    }
}
