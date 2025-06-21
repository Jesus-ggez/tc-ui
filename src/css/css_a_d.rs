use pyo3::prelude::*;

use crate::dom_components::StyleComponent;

//<·
#[pymethods]
impl StyleComponent {
    fn align_content(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "align-content".to_string(), value)
    }

    fn align_items(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "align-items".to_string(), value)
    }

    fn align_self(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "align-self".to_string(), value)
    }

    fn alignment_adjust(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "alignment-adjust".to_string(), value)
    }

    fn alignment_baseline(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "alignment-baseline".to_string(), value)
    }

    fn all(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "all".to_string(), value)
    }

    fn alt(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "alt".to_string(), value)
    }

    fn animation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation".to_string(), value)
    }

    fn animation_delay(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-delay".to_string(), value)
    }

    fn animation_direction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-direction".to_string(), value)
    }

    fn animation_duration(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-duration".to_string(), value)
    }

    fn animation_fill_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-fill-mode".to_string(), value)
    }

    fn animation_iteration_count(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-iteration-count".to_string(), value)
    }

    fn animation_name(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-name".to_string(), value)
    }

    fn animation_play_state(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-play-state".to_string(), value)
    }

    fn animation_timing_function(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-timing-function".to_string(), value)
    }

    fn azimuth(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "azimuth".to_string(), value)
    }

    fn backface_visibility(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "backface-visibility".to_string(), value)
    }

    fn background(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background".to_string(), value)
    }

    fn background_attachment(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-attachment".to_string(), value)
    }

    fn background_clip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-clip".to_string(), value)
    }

    fn background_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-color".to_string(), value)
    }

    fn background_image(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-image".to_string(), value)
    }

    fn background_origin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-origin".to_string(), value)
    }

    fn background_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-position".to_string(), value)
    }

    fn background_repeat(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-repeat".to_string(), value)
    }

    fn background_size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-size".to_string(), value)
    }

    fn background_blend_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-blend-mode".to_string(), value)
    }

    fn baseline_shift(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "baseline-shift".to_string(), value)
    }

    fn bleed(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bleed".to_string(), value)
    }

    fn bookmark_label(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bookmark-label".to_string(), value)
    }

    fn bookmark_level(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bookmark-level".to_string(), value)
    }

    fn bookmark_state(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bookmark-state".to_string(), value)
    }

    fn border(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border".to_string(), value)
    }

    fn border_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-color".to_string(), value)
    }

    fn border_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-style".to_string(), value)
    }

    fn border_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-width".to_string(), value)
    }

    fn border_bottom(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom".to_string(), value)
    }

    fn border_bottom_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-color".to_string(), value)
    }

    fn border_bottom_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-style".to_string(), value)
    }

    fn border_bottom_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-width".to_string(), value)
    }

    fn border_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-left".to_string(), value)
    }

    fn border_left_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-left-color".to_string(), value)
    }

    fn border_left_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-left-style".to_string(), value)
    }

    fn border_left_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-left-width".to_string(), value)
    }

    fn border_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-right".to_string(), value)
    }

    fn border_right_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-right-color".to_string(), value)
    }

    fn border_right_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-right-style".to_string(), value)
    }

    fn border_right_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-right-width".to_string(), value)
    }

    fn border_top(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top".to_string(), value)
    }

    fn border_top_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-color".to_string(), value)
    }

    fn border_top_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-style".to_string(), value)
    }

    fn border_top_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-width".to_string(), value)
    }

    fn border_collapse(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-collapse".to_string(), value)
    }

    fn border_image(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image".to_string(), value)
    }

    fn border_image_outset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-outset".to_string(), value)
    }

    fn border_image_repeat(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-repeat".to_string(), value)
    }

    fn border_image_slice(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-slice".to_string(), value)
    }

    fn border_image_source(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-source".to_string(), value)
    }

    fn border_image_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-width".to_string(), value)
    }

    fn border_radius(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-radius".to_string(), value)
    }

    fn border_bottom_left_radius(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-left-radius".to_string(), value)
    }

    fn border_bottom_right_radius(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-right-radius".to_string(), value)
    }

    fn border_top_left_radius(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-left-radius".to_string(), value)
    }

    fn border_top_right_radius(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-right-radius".to_string(), value)
    }

    fn border_spacing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-spacing".to_string(), value)
    }

    fn bottom(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bottom".to_string(), value)
    }

    fn box_decoration_break(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "box-decoration-break".to_string(), value)
    }

    fn box_shadow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "box-shadow".to_string(), value)
    }

    fn box_sizing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "box-sizing".to_string(), value)
    }

    fn box_snap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "box-snap".to_string(), value)
    }

    fn break_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "break-after".to_string(), value)
    }

    fn break_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "break-before".to_string(), value)
    }

    fn break_inside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "break-inside".to_string(), value)
    }

    fn buffered_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "buffered-rendering".to_string(), value)
    }

    fn caption_side(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "caption-side".to_string(), value)
    }

    fn clear(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clear".to_string(), value)
    }

    fn clear_side(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clear-side".to_string(), value)
    }

    fn clip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clip".to_string(), value)
    }

    fn clip_path(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clip-path".to_string(), value)
    }

    fn clip_rule(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clip-rule".to_string(), value)
    }

    fn color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color".to_string(), value)
    }

    fn color_adjust(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-adjust".to_string(), value)
    }

    fn color_correction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-correction".to_string(), value)
    }

    fn color_interpolation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-interpolation".to_string(), value)
    }

    fn color_interpolation_filters(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-interpolation-filters".to_string(), value)
    }

    fn color_profile(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-profile".to_string(), value)
    }

    fn color_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-rendering".to_string(), value)
    }

    fn column_fill(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-fill".to_string(), value)
    }

    fn column_gap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-gap".to_string(), value)
    }

    fn column_rule(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-rule".to_string(), value)
    }

    fn column_rule_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-rule-color".to_string(), value)
    }

    fn column_rule_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-rule-style".to_string(), value)
    }

    fn column_rule_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-rule-width".to_string(), value)
    }

    fn column_span(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-span".to_string(), value)
    }

    fn columns(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "columns".to_string(), value)
    }

    fn column_count(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-count".to_string(), value)
    }

    fn column_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-width".to_string(), value)
    }

    fn contain(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "contain".to_string(), value)
    }

    fn content(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "content".to_string(), value)
    }

    fn counter_increment(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "counter-increment".to_string(), value)
    }

    fn counter_reset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "counter-reset".to_string(), value)
    }

    fn counter_set(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "counter-set".to_string(), value)
    }

    fn cue(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "cue".to_string(), value)
    }

    fn cue_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "cue-after".to_string(), value)
    }

    fn cue_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "cue-before".to_string(), value)
    }

    fn cursor(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "cursor".to_string(), value)
    }

    fn direction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "direction".to_string(), value)
    }

    fn display(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display".to_string(), value)
    }

    fn display_inside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display-inside".to_string(), value)
    }

    fn display_outside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display-outside".to_string(), value)
    }

    fn display_extras(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display-extras".to_string(), value)
    }

    fn display_box(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display-box".to_string(), value)
    }

    fn dominant_baseline(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "dominant-baseline".to_string(), value)
    }
}
