use pyo3::prelude::*;

use crate::dom_components::StyleComponent;

//<·
#[pymethods]
impl StyleComponent {
    fn margin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin".to_string(), value)
    }

    fn margin_bottom(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin-bottom".to_string(), value)
    }

    fn margin_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin-left".to_string(), value)
    }

    fn margin_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin-right".to_string(), value)
    }

    fn margin_top(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin-top".to_string(), value)
    }

    fn marker(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker".to_string(), value)
    }

    fn marker_end(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-end".to_string(), value)
    }

    fn marker_mid(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-mid".to_string(), value)
    }

    fn marker_pattern(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-pattern".to_string(), value)
    }

    fn marker_segment(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-segment".to_string(), value)
    }

    fn marker_start(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-start".to_string(), value)
    }

    fn marker_knockout_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-knockout-left".to_string(), value)
    }

    fn marker_knockout_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-knockout-right".to_string(), value)
    }

    fn marker_side(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-side".to_string(), value)
    }

    fn marks(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marks".to_string(), value)
    }

    fn marquee_direction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marquee-direction".to_string(), value)
    }

    fn marquee_play_count(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marquee-play-count".to_string(), value)
    }

    fn marquee_speed(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marquee-speed".to_string(), value)
    }

    fn marquee_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marquee-style".to_string(), value)
    }

    fn mask(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask".to_string(), value)
    }

    fn mask_image(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-image".to_string(), value)
    }

    fn mask_repeat(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-repeat".to_string(), value)
    }

    fn mask_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-position".to_string(), value)
    }

    fn mask_clip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-clip".to_string(), value)
    }

    fn mask_origin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-origin".to_string(), value)
    }

    fn mask_size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-size".to_string(), value)
    }

    fn mask_box(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box".to_string(), value)
    }

    fn mask_box_outset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-outset".to_string(), value)
    }

    fn mask_box_repeat(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-repeat".to_string(), value)
    }

    fn mask_box_slice(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-slice".to_string(), value)
    }

    fn mask_box_source(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-source".to_string(), value)
    }

    fn mask_box_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-width".to_string(), value)
    }

    fn mask_type(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-type".to_string(), value)
    }

    fn max_height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "max-height".to_string(), value)
    }

    fn max_lines(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "max-lines".to_string(), value)
    }

    fn max_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "max-width".to_string(), value)
    }

    fn min_height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "min-height".to_string(), value)
    }

    fn min_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "min-width".to_string(), value)
    }

    fn mix_blend_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mix-blend-mode".to_string(), value)
    }

    fn nav_down(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-down".to_string(), value)
    }

    fn nav_index(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-index".to_string(), value)
    }

    fn nav_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-left".to_string(), value)
    }

    fn nav_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-right".to_string(), value)
    }

    fn nav_up(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-up".to_string(), value)
    }

    fn object_fit(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "object-fit".to_string(), value)
    }

    fn object_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "object-position".to_string(), value)
    }

    fn offafter(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "offset-after".to_string(), value)
    }

    fn offbefore(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "offset-before".to_string(), value)
    }

    fn offend(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "offset-end".to_string(), value)
    }

    fn offstart(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "offset-start".to_string(), value)
    }

    fn opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "opacity".to_string(), value)
    }

    fn order(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "order".to_string(), value)
    }

    fn orphans(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "orphans".to_string(), value)
    }

    fn outline(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline".to_string(), value)
    }

    fn outline_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline-color".to_string(), value)
    }

    fn outline_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline-style".to_string(), value)
    }

    fn outline_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline-width".to_string(), value)
    }

    fn outline_offset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline-offset".to_string(), value)
    }

    fn overflow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow".to_string(), value)
    }

    fn overflow_x(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow-x".to_string(), value)
    }

    fn overflow_y(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow-y".to_string(), value)
    }

    fn overflow_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow-style".to_string(), value)
    }

    fn overflow_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow-wrap".to_string(), value)
    }

    fn padding(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding".to_string(), value)
    }

    fn padding_bottom(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding-bottom".to_string(), value)
    }

    fn padding_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding-left".to_string(), value)
    }

    fn padding_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding-right".to_string(), value)
    }

    fn padding_top(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding-top".to_string(), value)
    }

    fn page(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "page".to_string(), value)
    }

    fn page_break_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "page-break-after".to_string(), value)
    }

    fn page_break_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "page-break-before".to_string(), value)
    }

    fn page_break_inside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "page-break-inside".to_string(), value)
    }

    fn paint_order(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "paint-order".to_string(), value)
    }

    fn pause(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pause".to_string(), value)
    }

    fn pause_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pause-after".to_string(), value)
    }

    fn pause_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pause-before".to_string(), value)
    }

    fn perspective(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "perspective".to_string(), value)
    }

    fn perspective_origin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "perspective-origin".to_string(), value)
    }

    fn pitch(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pitch".to_string(), value)
    }

    fn pitch_range(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pitch-range".to_string(), value)
    }

    fn play_during(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "play-during".to_string(), value)
    }

    fn pointer_events(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pointer-events".to_string(), value)
    }

    fn position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "position".to_string(), value)
    }


}
