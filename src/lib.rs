use pyo3::prelude::*;

//~>
mod dom_components;

//+?
mod css_properties;
mod html_properties;

//.?
use crate::dom_components::{HtmlElement, StyleComponent};

//<·
#[pymodule]
fn tc_ui(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let _ = m.add_class::<StyleComponent>()?;
    let _ = m.add_class::<HtmlElement>()?;
    Ok(())
}
