use pyo3::prelude::*;

use crate::dom_components::StyleComponent;
use crate::utils::formaters;

//<·
#[pymethods]
impl StyleComponent {
    fn as_class(&self, class_name: String) -> PyResult<String> {
        Ok(format!(".{} {}", class_name, self.__str__()?))
    }

    fn as_tag(&self, class_name: String) -> PyResult<String> {
        let fmt: Vec<String> = self
            .__simple_base_content()
            .iter()
            .map(|entry| format!("\t\t{}", entry))
            .collect();

        Ok(format!(
            "<style>\n\t{} {{\n{}\n}}\n</style>\n",
            class_name,
            fmt.join("\n")
        ))
    }

    fn as_inline(&self, use_attr: bool) -> PyResult<String> {
        let entries = self.__simple_base_content().join("");

        let prefix = "style=\"";

        Ok(match use_attr {
            true => prefix.to_owned() + &entries,
            false => entries,
        })
    }
    pub fn set_property(
        mut slf: PyRefMut<'_, Self>,
        name: String,
        value: String,
    ) -> PyRefMut<'_, Self> {
        let _ = slf
            .properties
            .insert(name, format!("{}", formaters::repr(&value)));
        slf
    }
}
