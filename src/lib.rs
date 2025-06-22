use pyo3::prelude::*;

//~>
mod dom_components;
mod critical;

//+?
mod css;
mod html;

mod tc_components;
mod utils;

//.?
use crate::dom_components::{HtmlElement, StyleComponent};
use crate::tc_components::*;

//<·
#[pymodule]
fn tc_ui(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let _ = m.add_class::<StyleComponent>()?;
    let _ = m.add_class::<HtmlElement>()?;
    let _ = m.add_class::<crate::text::Text>()?;
    Ok(())
}
