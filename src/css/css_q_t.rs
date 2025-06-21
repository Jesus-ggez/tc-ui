use pyo3::prelude::*;

use crate::dom_components::StyleComponent;

//<·
#[pymethods]
impl StyleComponent {
    fn quotes(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "quotes".to_string(), value)
    }

    fn region_fragment(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "region-fragment".to_string(), value)
    }

    fn resize(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "resize".to_string(), value)
    }

    fn rest(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "rest".to_string(), value)
    }

    fn rest_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "rest-after".to_string(), value)
    }

    fn rest_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "rest-before".to_string(), value)
    }

    fn richness(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "richness".to_string(), value)
    }

    fn right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "right".to_string(), value)
    }

    fn ruby_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ruby-align".to_string(), value)
    }

    fn ruby_merge(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ruby-merge".to_string(), value)
    }

    fn ruby_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ruby-position".to_string(), value)
    }

    fn scroll_behavior(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-behavior".to_string(), value)
    }

    fn scroll_snap_coordinate(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-coordinate".to_string(), value)
    }

    fn scroll_snap_destination(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-destination".to_string(), value)
    }

    fn scroll_snap_points_x(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-points-x".to_string(), value)
    }

    fn scroll_snap_points_y(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-points-y".to_string(), value)
    }

    fn scroll_snap_type(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-type".to_string(), value)
    }

    fn shape_image_threshold(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-image-threshold".to_string(), value)
    }

    fn shape_inside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-inside".to_string(), value)
    }

    fn shape_margin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-margin".to_string(), value)
    }

    fn shape_outside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-outside".to_string(), value)
    }

    fn shape_padding(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-padding".to_string(), value)
    }

    fn shape_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-rendering".to_string(), value)
    }

    fn size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "size".to_string(), value)
    }

    fn speak(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak".to_string(), value)
    }

    fn speak_as(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak-as".to_string(), value)
    }

    fn speak_header(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak-header".to_string(), value)
    }

    fn speak_numeral(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak-numeral".to_string(), value)
    }

    fn speak_punctuation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak-punctuation".to_string(), value)
    }

    fn speech_rate(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speech-rate".to_string(), value)
    }

    fn stop_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stop-color".to_string(), value)
    }

    fn stop_opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stop-opacity".to_string(), value)
    }

    fn stress(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stress".to_string(), value)
    }

    fn string_set(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "string-set".to_string(), value)
    }

    fn stroke(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke".to_string(), value)
    }

    fn stroke_dasharray(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-dasharray".to_string(), value)
    }

    fn stroke_dashoffset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-dashoffset".to_string(), value)
    }

    fn stroke_linecap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-linecap".to_string(), value)
    }

    fn stroke_linejoin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-linejoin".to_string(), value)
    }

    fn stroke_miterlimit(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-miterlimit".to_string(), value)
    }

    fn stroke_opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-opacity".to_string(), value)
    }

    fn stroke_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-width".to_string(), value)
    }

    fn tab_size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "tab-size".to_string(), value)
    }

    fn table_layout(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "table-layout".to_string(), value)
    }

    fn text_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-align".to_string(), value)
    }

    fn text_align_all(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-align-all".to_string(), value)
    }

    fn text_align_last(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-align-last".to_string(), value)
    }

    fn text_anchor(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-anchor".to_string(), value)
    }

    fn text_combine_upright(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-combine-upright".to_string(), value)
    }

    fn text_decoration(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration".to_string(), value)
    }

    fn text_decoration_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration-color".to_string(), value)
    }

    fn text_decoration_line(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration-line".to_string(), value)
    }

    fn text_decoration_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration-style".to_string(), value)
    }

    fn text_decoration_skip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration-skip".to_string(), value)
    }

    fn text_emphasis(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis".to_string(), value)
    }

    fn text_emphasis_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis-color".to_string(), value)
    }

    fn text_emphasis_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis-style".to_string(), value)
    }

    fn text_emphasis_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis-position".to_string(), value)
    }

    fn text_emphasis_skip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis-skip".to_string(), value)
    }

    fn text_height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-height".to_string(), value)
    }

    fn text_indent(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-indent".to_string(), value)
    }

    fn text_justify(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-justify".to_string(), value)
    }

    fn text_orientation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-orientation".to_string(), value)
    }

    fn text_overflow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-overflow".to_string(), value)
    }

    fn text_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-rendering".to_string(), value)
    }

    fn text_shadow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-shadow".to_string(), value)
    }

    fn text_size_adjust(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-size-adjust".to_string(), value)
    }

    fn text_space_collapse(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-space-collapse".to_string(), value)
    }

    fn text_spacing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-spacing".to_string(), value)
    }

    fn text_transform(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-transform".to_string(), value)
    }

    fn text_underline_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-underline-position".to_string(), value)
    }

    fn text_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-wrap".to_string(), value)
    }

    fn top(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "top".to_string(), value)
    }

    fn touch_action(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "touch-action".to_string(), value)
    }

    fn transform(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transform".to_string(), value)
    }

    fn transform_box(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transform-box".to_string(), value)
    }

    fn transform_origin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transform-origin".to_string(), value)
    }

    fn transform_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transform-style".to_string(), value)
    }

    fn transition(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition".to_string(), value)
    }

    fn transition_delay(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition-delay".to_string(), value)
    }

    fn transition_duration(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition-duration".to_string(), value)
    }

    fn transition_set_property(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition-set_property".to_string(), value)
    }


}
