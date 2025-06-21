use pyo3::prelude::*;

use crate::dom_components::StyleComponent;

//<·
#[pymethods]
impl StyleComponent {
    fn unicode_bidi(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "unicode-bidi".to_string(), value)
    }

    fn vector_effect(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "vector-effect".to_string(), value)
    }

    fn vertical_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "vertical-align".to_string(), value)
    }

    fn visibility(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "visibility".to_string(), value)
    }

    fn voice_balance(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-balance".to_string(), value)
    }

    fn voice_duration(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-duration".to_string(), value)
    }

    fn voice_family(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-family".to_string(), value)
    }

    fn voice_pitch(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-pitch".to_string(), value)
    }

    fn voice_range(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-range".to_string(), value)
    }

    fn voice_rate(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-rate".to_string(), value)
    }

    fn voice_stress(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-stress".to_string(), value)
    }

    fn voice_volumn(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-volumn".to_string(), value)
    }

    fn volume(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "volume".to_string(), value)
    }

    fn white_space(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "white-space".to_string(), value)
    }

    fn widows(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "widows".to_string(), value)
    }

    fn width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "width".to_string(), value)
    }

    fn will_change(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "will-change".to_string(), value)
    }

    fn word_break(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "word-break".to_string(), value)
    }

    fn word_spacing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "word-spacing".to_string(), value)
    }

    fn word_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "word-wrap".to_string(), value)
    }

    fn wrap_flow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "wrap-flow".to_string(), value)
    }

    fn wrap_through(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "wrap-through".to_string(), value)
    }

    fn writing_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "writing-mode".to_string(), value)
    }

    fn z_index(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "z-index".to_string(), value)
    }

    fn transition_timing_function(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition-timing-function".to_string(), value)
    }

}
