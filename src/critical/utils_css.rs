use crate::dom_components::StyleComponent;

//<·
impl StyleComponent {
    pub fn __list_properties(&self) -> Vec<String> {
        let mut css = Vec::new();

        if self.properties.is_empty() {
            return css;
        }

        for (k, v) in &self.properties {
            css.push(format!("{}: {};", k, v));
        }
        css
    }

    pub fn __list_tab_properties(&self) -> Vec<String> {
        self
            .__list_properties()
            .iter()
            .map(|line| "\t".to_owned() + line)
            .collect()
    }
}
