use pyo3::prelude::*;


mod static_component;
mod css_properties;


use crate::static_component::StaticComponent;


//<·
#[pymodule]
fn tc_ui(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let _ = m.add_class::<StaticComponent>()?;
    Ok(())
}
