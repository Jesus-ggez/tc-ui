use pyo3::prelude::*;
use std::collections::HashMap;

use crate::dom_components::StyleComponent;
use crate::utils::formaters;

//<·
#[pymethods]
impl StyleComponent {
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
    pub fn update(&mut self, value: HashMap<String, String>) {
        self.properties.extend(value)
    }
}
