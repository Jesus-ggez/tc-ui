use pyo3::prelude::*;

use crate::dom_components::HtmlElement;
use crate::utils::str_replace::replace;

//<·
#[pyclass(extends=HtmlElement, subclass)]
pub struct Commentary {
    #[pyo3(get, set)]
    value: String,
}

#[pymethods]
impl Commentary {
    #[new]
    #[pyo3(signature=(value = None))]
    fn new(
        value: Option<String>,
    ) -> (Self, HtmlElement) {
        let new_value: String = value.unwrap_or_else(|| "".to_string());
        let slf_content = Commentary {
            value: new_value.clone(),
        };
        let mut supr_content = HtmlElement::new("!--".to_string(), None);

        supr_content.add_html(new_value);
        (slf_content, supr_content)
    }

    #[pyo3(signature=(value = None, tag = None))]
    fn __init__(mut slf: PyRefMut<'_, Self>, value: Option<String>, tag: Option<String>) {
        slf.value = value.unwrap_or_default();
        slf.as_super().components = vec![slf.value.clone()];

        slf.as_super().tag = tag.filter(|t| !t.is_empty()).unwrap_or_default();
    }

    pub fn __str__(slf: PyRef<'_, Self>) -> PyResult<String> {
        let supr = slf.as_super();
        let tag = supr.__str__()?;

        Ok(
            {
                let comm = replace(&tag, "</!", "", Some(2));
                replace(&comm, "<!-->", "<!--", None)
            }
        )
    }

    #[setter]
    fn set_value(mut slf: PyRefMut<'_, Self>, value: String) {
        slf.value = value;
        slf.as_super().components = vec![slf.value.clone()];
    }

    fn formated(&self) {}

    fn as_tag(&self) {}
}

