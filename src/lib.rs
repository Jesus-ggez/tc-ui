use pyo3::prelude::*;

//~>
mod dom_components;
mod critical;

//+?
mod css;
mod html;

mod utils;

//mod tc_components;
//use crate::tc_components::*;
//.?
use crate::dom_components::{HtmlElement, StyleComponent};

//<·
#[pymodule]
fn tc_ui(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let _ = m.add_class::<StyleComponent>()?;
    let _ = m.add_class::<HtmlElement>()?;
    //let _ = m.add_class::<crate::text::Text>()?;
    //let _ = m.add_class::<crate::commentary::Commentary>()?;
    Ok(())
}
