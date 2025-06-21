use pyo3::prelude::*;

use crate::dom_components::StyleComponent;

//<·
#[pymethods]
impl StyleComponent {
    fn icon(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "icon".to_string(), value)
    }

    fn image_orientation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "image-orientation".to_string(), value)
    }

    fn image_resolution(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "image-resolution".to_string(), value)
    }

    fn image_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "image-rendering".to_string(), value)
    }

    fn ime(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime".to_string(), value)
    }

    fn ime_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime-align".to_string(), value)
    }

    fn ime_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime-mode".to_string(), value)
    }

    fn ime_offset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime-offset".to_string(), value)
    }

    fn ime_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime-width".to_string(), value)
    }

    fn initial_letters(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "initial-letters".to_string(), value)
    }

    fn inline_box_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "inline-box-align".to_string(), value)
    }

    fn isolation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "isolation".to_string(), value)
    }

    fn justify_content(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "justify-content".to_string(), value)
    }

    fn justify_items(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "justify-items".to_string(), value)
    }

    fn justify_self(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "justify-self".to_string(), value)
    }

    fn kerning(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "kerning".to_string(), value)
    }

    fn left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "left".to_string(), value)
    }

    fn letter_spacing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "letter-spacing".to_string(), value)
    }

    fn lighting_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "lighting-color".to_string(), value)
    }

    fn line_box_contain(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-box-contain".to_string(), value)
    }

    fn line_break(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-break".to_string(), value)
    }

    fn line_grid(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-grid".to_string(), value)
    }

    fn line_height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-height".to_string(), value)
    }

    fn line_slack(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-slack".to_string(), value)
    }

    fn line_snap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-snap".to_string(), value)
    }

    fn list_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "list-style".to_string(), value)
    }

    fn list_style_image(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "list-style-image".to_string(), value)
    }

    fn list_style_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "list-style-position".to_string(), value)
    }

    fn list_style_type(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "list-style-type".to_string(), value)
    }
}
