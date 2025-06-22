use crate::critical::hierarchy::Hierarchy;
use crate::utils::str_replace::replace;
use crate::dom_components::{HtmlElement, StyleComponent};

//<·
impl Hierarchy for StyleComponent {
    fn decompose(&self) -> Vec<String> {
        self.__simple_base_content()
    }
    fn compose(&self) -> Vec<String> {
        vec!["{".to_string()]
            .into_iter()
            .chain(self.__indent_content())
            .chain(["}".to_string()])
            .collect()
    }
}

impl Hierarchy for HtmlElement {
    fn decompose(&self) -> Vec<String> {
        let mut attrs: Vec<String> = self
            .__list_attrs()
            .iter()
            .map(|c| format!("\t{}", c))
            .collect();

        let tab_components: Vec<String> = self.__list_tab_components()
            .iter()
            .map(|c| replace(c, "\n", "\n\t", None))
            .collect();

        if attrs.is_empty() && tab_components.is_empty() {
            return vec![self.__str__().unwrap()];
        }

        let mut space = ">".to_string();

        if !attrs.is_empty() {
            space = " ".to_string();
            attrs.push(">".to_string())
        }

        vec![format!("<{}{}", self.tag, space)]
            .into_iter()
            .chain(attrs)
            .chain(tab_components)
            .chain(vec![format!("</{}>", self.tag)])
            .collect()
    }

    fn compose(&self) -> Vec<String> {
        self.decompose()
            .iter()
            .map(|c| format!("\t{}", c))
            .collect()
    }
}
