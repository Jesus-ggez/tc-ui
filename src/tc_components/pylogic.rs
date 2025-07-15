use pyo3::prelude::*;


//<···>
#[pyclass]
pub struct Page;


#[pymethods]
impl Page {
    fn create_styles(&self, raw_template: String) {
        raw_template
            .iter()
            .map(|tag| )
    }
}
