use pyo3::prelude::*;

//~>
mod dom_components;
mod critical;

//+?
mod css;
mod html;

mod utils;

mod tc_components;

//.?
use crate::dom_components::{HtmlElement, StyleComponent};
use crate::tc_components::*;
use crate::css::*;

//<·
#[pymodule]
fn tc_ui(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let _ = m.add_class::<crate::text::Text>()?;

    // ~> alignment
    /*
    let _ = m.add_class::<crate::enums::JustifyContent>()?;
    let _ = m.add_class::<crate::enums::AlignContent>()?;
    let _ = m.add_class::<crate::enums::AlignItems>()?;
    */

    let _ = m.add_class::<StyleComponent>()?;
    let _ = m.add_class::<HtmlElement>()?;
    Ok(())
}

