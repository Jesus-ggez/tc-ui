use pyo3::prelude::*;
use std::collections::HashMap;

use crate::static_component::StaticComponent;

#[pymethods]
impl StaticComponent {
    #[new]
    fn new() -> Self {
        StaticComponent {
            properties: HashMap::new(),
        }
    }
    fn set_property(
        mut slf: PyRefMut<'_, Self>,
        name: String,
        value: String,
    ) -> PyRefMut<'_, Self> {
        let _ = slf.properties.insert(name, value);
        slf
    }

    // set align-content
    fn set_align_content(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "align-content".to_string(), value)
    }

    // set align-items
    fn set_align_items(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "align-items".to_string(), value)
    }

    // set align-self
    fn set_align_self(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "align-self".to_string(), value)
    }

    // set alignment-adjust
    fn set_alignment_adjust(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "alignment-adjust".to_string(), value)
    }

    // set alignment-baseline
    fn set_alignment_baseline(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "alignment-baseline".to_string(), value)
    }

    // set all
    fn set_all(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "all".to_string(), value)
    }

    // set alt
    fn set_alt(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "alt".to_string(), value)
    }

    // set animation
    fn set_animation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation".to_string(), value)
    }

    // set animation-delay
    fn set_animation_delay(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-delay".to_string(), value)
    }

    // set animation-direction
    fn set_animation_direction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-direction".to_string(), value)
    }

    // set animation-duration
    fn set_animation_duration(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-duration".to_string(), value)
    }

    // set animation-fill-mode
    fn set_animation_fill_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-fill-mode".to_string(), value)
    }

    // set animation-iteration-count
    fn set_animation_iteration_count(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-iteration-count".to_string(), value)
    }

    // set animation-name
    fn set_animation_name(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-name".to_string(), value)
    }

    // set animation-play-state
    fn set_animation_play_state(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-play-state".to_string(), value)
    }

    // set animation-timing-function
    fn set_animation_timing_function(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-timing-function".to_string(), value)
    }

    // set azimuth
    fn set_azimuth(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "azimuth".to_string(), value)
    }

    // set backface-visibility
    fn set_backface_visibility(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "backface-visibility".to_string(), value)
    }

    // set background
    fn set_background(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background".to_string(), value)
    }

    // set background-attachment
    fn set_background_attachment(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-attachment".to_string(), value)
    }

    // set background-clip
    fn set_background_clip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-clip".to_string(), value)
    }

    // set background-color
    fn set_background_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-color".to_string(), value)
    }

    // set background-image
    fn set_background_image(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-image".to_string(), value)
    }

    // set background-origin
    fn set_background_origin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-origin".to_string(), value)
    }

    // set background-position
    fn set_background_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-position".to_string(), value)
    }

    // set background-repeat
    fn set_background_repeat(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-repeat".to_string(), value)
    }

    // set background-size
    fn set_background_size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-size".to_string(), value)
    }

    // set background-blend-mode
    fn set_background_blend_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-blend-mode".to_string(), value)
    }

    // set baseline-shift
    fn set_baseline_shift(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "baseline-shift".to_string(), value)
    }

    // set bleed
    fn set_bleed(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bleed".to_string(), value)
    }

    // set bookmark-label
    fn set_bookmark_label(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bookmark-label".to_string(), value)
    }

    // set bookmark-level
    fn set_bookmark_level(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bookmark-level".to_string(), value)
    }

    // set bookmark-state
    fn set_bookmark_state(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bookmark-state".to_string(), value)
    }

    // set border
    fn set_border(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border".to_string(), value)
    }

    // set border-color
    fn set_border_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-color".to_string(), value)
    }

    // set border-style
    fn set_border_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-style".to_string(), value)
    }

    // set border-width
    fn set_border_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-width".to_string(), value)
    }

    // set border-bottom
    fn set_border_bottom(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom".to_string(), value)
    }

    // set border-bottom-color
    fn set_border_bottom_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-color".to_string(), value)
    }

    // set border-bottom-style
    fn set_border_bottom_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-style".to_string(), value)
    }

    // set border-bottom-width
    fn set_border_bottom_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-width".to_string(), value)
    }

    // set border-left
    fn set_border_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-left".to_string(), value)
    }

    // set border-left-color
    fn set_border_left_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-left-color".to_string(), value)
    }

    // set border-left-style
    fn set_border_left_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-left-style".to_string(), value)
    }

    // set border-left-width
    fn set_border_left_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-left-width".to_string(), value)
    }

    // set border-right
    fn set_border_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-right".to_string(), value)
    }

    // set border-right-color
    fn set_border_right_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-right-color".to_string(), value)
    }

    // set border-right-style
    fn set_border_right_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-right-style".to_string(), value)
    }

    // set border-right-width
    fn set_border_right_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-right-width".to_string(), value)
    }

    // set border-top
    fn set_border_top(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top".to_string(), value)
    }

    // set border-top-color
    fn set_border_top_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-color".to_string(), value)
    }

    // set border-top-style
    fn set_border_top_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-style".to_string(), value)
    }

    // set border-top-width
    fn set_border_top_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-width".to_string(), value)
    }

    // set border-collapse
    fn set_border_collapse(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-collapse".to_string(), value)
    }

    // set border-image
    fn set_border_image(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image".to_string(), value)
    }

    // set border-image-outset
    fn set_border_image_outset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-outset".to_string(), value)
    }

    // set border-image-repeat
    fn set_border_image_repeat(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-repeat".to_string(), value)
    }

    // set border-image-slice
    fn set_border_image_slice(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-slice".to_string(), value)
    }

    // set border-image-source
    fn set_border_image_source(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-source".to_string(), value)
    }

    // set border-image-width
    fn set_border_image_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-width".to_string(), value)
    }

    // set border-radius
    fn set_border_radius(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-radius".to_string(), value)
    }

    // set border-bottom-left-radius
    fn set_border_bottom_left_radius(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-left-radius".to_string(), value)
    }

    // set border-bottom-right-radius
    fn set_border_bottom_right_radius(
        slf: PyRefMut<'_, Self>,
        value: String,
    ) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-right-radius".to_string(), value)
    }

    // set border-top-left-radius
    fn set_border_top_left_radius(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-left-radius".to_string(), value)
    }

    // set border-top-right-radius
    fn set_border_top_right_radius(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-right-radius".to_string(), value)
    }

    // set border-spacing
    fn set_border_spacing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-spacing".to_string(), value)
    }

    // set bottom
    fn set_bottom(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bottom".to_string(), value)
    }

    // set box-decoration-break
    fn set_box_decoration_break(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "box-decoration-break".to_string(), value)
    }

    // set box-shadow
    fn set_box_shadow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "box-shadow".to_string(), value)
    }

    // set box-sizing
    fn set_box_sizing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "box-sizing".to_string(), value)
    }

    // set box-snap
    fn set_box_snap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "box-snap".to_string(), value)
    }

    // set break-after
    fn set_break_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "break-after".to_string(), value)
    }

    // set break-before
    fn set_break_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "break-before".to_string(), value)
    }

    // set break-inside
    fn set_break_inside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "break-inside".to_string(), value)
    }

    // set buffered-rendering
    fn set_buffered_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "buffered-rendering".to_string(), value)
    }

    // set caption-side
    fn set_caption_side(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "caption-side".to_string(), value)
    }

    // set clear
    fn set_clear(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clear".to_string(), value)
    }

    // set clear-side
    fn set_clear_side(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clear-side".to_string(), value)
    }

    // set clip
    fn set_clip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clip".to_string(), value)
    }

    // set clip-path
    fn set_clip_path(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clip-path".to_string(), value)
    }

    // set clip-rule
    fn set_clip_rule(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clip-rule".to_string(), value)
    }

    // set color
    fn set_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color".to_string(), value)
    }

    // set color-adjust
    fn set_color_adjust(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-adjust".to_string(), value)
    }

    // set color-correction
    fn set_color_correction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-correction".to_string(), value)
    }

    // set color-interpolation
    fn set_color_interpolation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-interpolation".to_string(), value)
    }

    // set color-interpolation-filters
    fn set_color_interpolation_filters(
        slf: PyRefMut<'_, Self>,
        value: String,
    ) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-interpolation-filters".to_string(), value)
    }

    // set color-profile
    fn set_color_profile(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-profile".to_string(), value)
    }

    // set color-rendering
    fn set_color_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-rendering".to_string(), value)
    }

    // set column-fill
    fn set_column_fill(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-fill".to_string(), value)
    }

    // set column-gap
    fn set_column_gap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-gap".to_string(), value)
    }

    // set column-rule
    fn set_column_rule(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-rule".to_string(), value)
    }

    // set column-rule-color
    fn set_column_rule_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-rule-color".to_string(), value)
    }

    // set column-rule-style
    fn set_column_rule_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-rule-style".to_string(), value)
    }

    // set column-rule-width
    fn set_column_rule_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-rule-width".to_string(), value)
    }

    // set column-span
    fn set_column_span(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-span".to_string(), value)
    }

    // set columns
    fn set_columns(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "columns".to_string(), value)
    }

    // set column-count
    fn set_column_count(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-count".to_string(), value)
    }

    // set column-width
    fn set_column_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-width".to_string(), value)
    }

    // set contain
    fn set_contain(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "contain".to_string(), value)
    }

    // set content
    fn set_content(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "content".to_string(), value)
    }

    // set counter-increment
    fn set_counter_increment(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "counter-increment".to_string(), value)
    }

    // set counter-reset
    fn set_counter_reset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "counter-reset".to_string(), value)
    }

    // set counter-set
    fn set_counter_set(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "counter-set".to_string(), value)
    }

    // set cue
    fn set_cue(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "cue".to_string(), value)
    }

    // set cue-after
    fn set_cue_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "cue-after".to_string(), value)
    }

    // set cue-before
    fn set_cue_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "cue-before".to_string(), value)
    }

    // set cursor
    fn set_cursor(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "cursor".to_string(), value)
    }

    // set direction
    fn set_direction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "direction".to_string(), value)
    }

    // set display
    fn set_display(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display".to_string(), value)
    }

    // set display-inside
    fn set_display_inside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display-inside".to_string(), value)
    }

    // set display-outside
    fn set_display_outside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display-outside".to_string(), value)
    }

    // set display-extras
    fn set_display_extras(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display-extras".to_string(), value)
    }

    // set display-box
    fn set_display_box(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display-box".to_string(), value)
    }

    // set dominant-baseline
    fn set_dominant_baseline(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "dominant-baseline".to_string(), value)
    }

    // set elevation
    fn set_elevation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "elevation".to_string(), value)
    }

    // set empty-cells
    fn set_empty_cells(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "empty-cells".to_string(), value)
    }

    // set enable-background
    fn set_enable_background(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "enable-background".to_string(), value)
    }

    // set fill
    fn set_fill(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "fill".to_string(), value)
    }

    // set fill-opacity
    fn set_fill_opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "fill-opacity".to_string(), value)
    }

    // set fill-rule
    fn set_fill_rule(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "fill-rule".to_string(), value)
    }

    // set filter
    fn set_filter(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "filter".to_string(), value)
    }

    // set float
    fn set_float(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float".to_string(), value)
    }

    // set float-defer-column
    fn set_float_defer_column(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float-defer-column".to_string(), value)
    }

    // set float-defer-page
    fn set_float_defer_page(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float-defer-page".to_string(), value)
    }

    // set float-offset
    fn set_float_offset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float-offset".to_string(), value)
    }

    // set float-wrap
    fn set_float_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float-wrap".to_string(), value)
    }

    // set flow-into
    fn set_flow_into(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flow-into".to_string(), value)
    }

    // set flow-from
    fn set_flow_from(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flow-from".to_string(), value)
    }

    // set flex
    fn set_flex(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex".to_string(), value)
    }

    // set flex-basis
    fn set_flex_basis(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-basis".to_string(), value)
    }

    // set flex-grow
    fn set_flex_grow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-grow".to_string(), value)
    }

    // set flex-shrink
    fn set_flex_shrink(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-shrink".to_string(), value)
    }

    // set flex-flow
    fn set_flex_flow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-flow".to_string(), value)
    }

    // set flex-direction
    fn set_flex_direction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-direction".to_string(), value)
    }

    // set flex-wrap
    fn set_flex_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-wrap".to_string(), value)
    }

    // set flood-color
    fn set_flood_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flood-color".to_string(), value)
    }

    // set flood-opacity
    fn set_flood_opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flood-opacity".to_string(), value)
    }

    // set font
    fn set_font(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font".to_string(), value)
    }

    // set font-family
    fn set_font_family(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-family".to_string(), value)
    }

    // set font-size
    fn set_font_size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-size".to_string(), value)
    }

    // set font-stretch
    fn set_font_stretch(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-stretch".to_string(), value)
    }

    // set font-style
    fn set_font_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-style".to_string(), value)
    }

    // set font-weight
    fn set_font_weight(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-weight".to_string(), value)
    }

    // set font-feature-settings
    fn set_font_feature_settings(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-feature-settings".to_string(), value)
    }

    // set font-kerning
    fn set_font_kerning(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-kerning".to_string(), value)
    }

    // set font-language-override
    fn set_font_language_override(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-language-override".to_string(), value)
    }

    // set font-size-adjust
    fn set_font_size_adjust(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-size-adjust".to_string(), value)
    }

    // set font-synthesis
    fn set_font_synthesis(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-synthesis".to_string(), value)
    }

    // set font-variant
    fn set_font_variant(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant".to_string(), value)
    }

    // set font-variant-alternates
    fn set_font_variant_alternates(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-alternates".to_string(), value)
    }

    // set font-variant-caps
    fn set_font_variant_caps(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-caps".to_string(), value)
    }

    // set font-variant-east-asian
    fn set_font_variant_east_asian(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-east-asian".to_string(), value)
    }

    // set font-variant-ligatures
    fn set_font_variant_ligatures(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-ligatures".to_string(), value)
    }

    // set font-variant-numeric
    fn set_font_variant_numeric(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-numeric".to_string(), value)
    }

    // set font-variant-position
    fn set_font_variant_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-position".to_string(), value)
    }

    // set footnote-policy
    fn set_footnote_policy(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "footnote-policy".to_string(), value)
    }

    // set glyph-orientation-horizontal
    fn set_glyph_orientation_horizontal(
        slf: PyRefMut<'_, Self>,
        value: String,
    ) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "glyph-orientation-horizontal".to_string(), value)
    }

    // set glyph-orientation-vertical
    fn set_glyph_orientation_vertical(
        slf: PyRefMut<'_, Self>,
        value: String,
    ) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "glyph-orientation-vertical".to_string(), value)
    }

    // set grid
    fn set_grid(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid".to_string(), value)
    }

    // set grid-auto-flow
    fn set_grid_auto_flow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-auto-flow".to_string(), value)
    }

    // set grid-auto-columns
    fn set_grid_auto_columns(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-auto-columns".to_string(), value)
    }

    // set grid-auto-rows
    fn set_grid_auto_rows(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-auto-rows".to_string(), value)
    }

    // set grid-template
    fn set_grid_template(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-template".to_string(), value)
    }

    // set grid-template-areas
    fn set_grid_template_areas(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-template-areas".to_string(), value)
    }

    // set grid-template-columns
    fn set_grid_template_columns(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-template-columns".to_string(), value)
    }

    // set grid-template-rows
    fn set_grid_template_rows(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-template-rows".to_string(), value)
    }

    // set grid-area
    fn set_grid_area(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-area".to_string(), value)
    }

    // set grid-column
    fn set_grid_column(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-column".to_string(), value)
    }

    // set grid-column-start
    fn set_grid_column_start(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-column-start".to_string(), value)
    }

    // set grid-column-end
    fn set_grid_column_end(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-column-end".to_string(), value)
    }

    // set grid-row
    fn set_grid_row(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-row".to_string(), value)
    }

    // set grid-row-start
    fn set_grid_row_start(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-row-start".to_string(), value)
    }

    // set grid-row-end
    fn set_grid_row_end(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-row-end".to_string(), value)
    }

    // set hanging-punctuation
    fn set_hanging_punctuation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hanging-punctuation".to_string(), value)
    }

    // set height
    fn set_height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "height".to_string(), value)
    }

    // set hyphenate-character
    fn set_hyphenate_character(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-character".to_string(), value)
    }

    // set hyphenate-limit-chars
    fn set_hyphenate_limit_chars(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-limit-chars".to_string(), value)
    }

    // set hyphenate-limit-last
    fn set_hyphenate_limit_last(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-limit-last".to_string(), value)
    }

    // set hyphenate-limit-lines
    fn set_hyphenate_limit_lines(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-limit-lines".to_string(), value)
    }

    // set hyphenate-limit-zone
    fn set_hyphenate_limit_zone(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-limit-zone".to_string(), value)
    }

    // set hyphens
    fn set_hyphens(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphens".to_string(), value)
    }

    // set icon
    fn set_icon(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "icon".to_string(), value)
    }

    // set image-orientation
    fn set_image_orientation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "image-orientation".to_string(), value)
    }

    // set image-resolution
    fn set_image_resolution(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "image-resolution".to_string(), value)
    }

    // set image-rendering
    fn set_image_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "image-rendering".to_string(), value)
    }

    // set ime
    fn set_ime(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime".to_string(), value)
    }

    // set ime-align
    fn set_ime_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime-align".to_string(), value)
    }

    // set ime-mode
    fn set_ime_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime-mode".to_string(), value)
    }

    // set ime-offset
    fn set_ime_offset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime-offset".to_string(), value)
    }

    // set ime-width
    fn set_ime_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime-width".to_string(), value)
    }

    // set initial-letters
    fn set_initial_letters(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "initial-letters".to_string(), value)
    }

    // set inline-box-align
    fn set_inline_box_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "inline-box-align".to_string(), value)
    }

    // set isolation
    fn set_isolation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "isolation".to_string(), value)
    }

    // set justify-content
    fn set_justify_content(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "justify-content".to_string(), value)
    }

    // set justify-items
    fn set_justify_items(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "justify-items".to_string(), value)
    }

    // set justify-self
    fn set_justify_self(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "justify-self".to_string(), value)
    }

    // set kerning
    fn set_kerning(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "kerning".to_string(), value)
    }

    // set left
    fn set_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "left".to_string(), value)
    }

    // set letter-spacing
    fn set_letter_spacing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "letter-spacing".to_string(), value)
    }

    // set lighting-color
    fn set_lighting_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "lighting-color".to_string(), value)
    }

    // set line-box-contain
    fn set_line_box_contain(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-box-contain".to_string(), value)
    }

    // set line-break
    fn set_line_break(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-break".to_string(), value)
    }

    // set line-grid
    fn set_line_grid(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-grid".to_string(), value)
    }

    // set line-height
    fn set_line_height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-height".to_string(), value)
    }

    // set line-slack
    fn set_line_slack(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-slack".to_string(), value)
    }

    // set line-snap
    fn set_line_snap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-snap".to_string(), value)
    }

    // set list-style
    fn set_list_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "list-style".to_string(), value)
    }

    // set list-style-image
    fn set_list_style_image(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "list-style-image".to_string(), value)
    }

    // set list-style-position
    fn set_list_style_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "list-style-position".to_string(), value)
    }

    // set list-style-type
    fn set_list_style_type(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "list-style-type".to_string(), value)
    }

    // set margin
    fn set_margin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin".to_string(), value)
    }

    // set margin-bottom
    fn set_margin_bottom(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin-bottom".to_string(), value)
    }

    // set margin-left
    fn set_margin_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin-left".to_string(), value)
    }

    // set margin-right
    fn set_margin_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin-right".to_string(), value)
    }

    // set margin-top
    fn set_margin_top(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin-top".to_string(), value)
    }

    // set marker
    fn set_marker(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker".to_string(), value)
    }

    // set marker-end
    fn set_marker_end(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-end".to_string(), value)
    }

    // set marker-mid
    fn set_marker_mid(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-mid".to_string(), value)
    }

    // set marker-pattern
    fn set_marker_pattern(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-pattern".to_string(), value)
    }

    // set marker-segment
    fn set_marker_segment(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-segment".to_string(), value)
    }

    // set marker-start
    fn set_marker_start(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-start".to_string(), value)
    }

    // set marker-knockout-left
    fn set_marker_knockout_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-knockout-left".to_string(), value)
    }

    // set marker-knockout-right
    fn set_marker_knockout_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-knockout-right".to_string(), value)
    }

    // set marker-side
    fn set_marker_side(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-side".to_string(), value)
    }

    // set marks
    fn set_marks(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marks".to_string(), value)
    }

    // set marquee-direction
    fn set_marquee_direction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marquee-direction".to_string(), value)
    }

    // set marquee-play-count
    fn set_marquee_play_count(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marquee-play-count".to_string(), value)
    }

    // set marquee-speed
    fn set_marquee_speed(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marquee-speed".to_string(), value)
    }

    // set marquee-style
    fn set_marquee_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marquee-style".to_string(), value)
    }

    // set mask
    fn set_mask(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask".to_string(), value)
    }

    // set mask-image
    fn set_mask_image(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-image".to_string(), value)
    }

    // set mask-repeat
    fn set_mask_repeat(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-repeat".to_string(), value)
    }

    // set mask-position
    fn set_mask_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-position".to_string(), value)
    }

    // set mask-clip
    fn set_mask_clip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-clip".to_string(), value)
    }

    // set mask-origin
    fn set_mask_origin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-origin".to_string(), value)
    }

    // set mask-size
    fn set_mask_size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-size".to_string(), value)
    }

    // set mask-box
    fn set_mask_box(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box".to_string(), value)
    }

    // set mask-box-outset
    fn set_mask_box_outset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-outset".to_string(), value)
    }

    // set mask-box-repeat
    fn set_mask_box_repeat(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-repeat".to_string(), value)
    }

    // set mask-box-slice
    fn set_mask_box_slice(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-slice".to_string(), value)
    }

    // set mask-box-source
    fn set_mask_box_source(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-source".to_string(), value)
    }

    // set mask-box-width
    fn set_mask_box_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-width".to_string(), value)
    }

    // set mask-type
    fn set_mask_type(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-type".to_string(), value)
    }

    // set max-height
    fn set_max_height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "max-height".to_string(), value)
    }

    // set max-lines
    fn set_max_lines(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "max-lines".to_string(), value)
    }

    // set max-width
    fn set_max_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "max-width".to_string(), value)
    }

    // set min-height
    fn set_min_height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "min-height".to_string(), value)
    }

    // set min-width
    fn set_min_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "min-width".to_string(), value)
    }

    // set mix-blend-mode
    fn set_mix_blend_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mix-blend-mode".to_string(), value)
    }

    // set nav-down
    fn set_nav_down(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-down".to_string(), value)
    }

    // set nav-index
    fn set_nav_index(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-index".to_string(), value)
    }

    // set nav-left
    fn set_nav_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-left".to_string(), value)
    }

    // set nav-right
    fn set_nav_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-right".to_string(), value)
    }

    // set nav-up
    fn set_nav_up(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-up".to_string(), value)
    }

    // set object-fit
    fn set_object_fit(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "object-fit".to_string(), value)
    }

    // set object-position
    fn set_object_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "object-position".to_string(), value)
    }

    // set offset-after
    fn set_offset_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "offset-after".to_string(), value)
    }

    // set offset-before
    fn set_offset_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "offset-before".to_string(), value)
    }

    // set offset-end
    fn set_offset_end(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "offset-end".to_string(), value)
    }

    // set offset-start
    fn set_offset_start(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "offset-start".to_string(), value)
    }

    // set opacity
    fn set_opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "opacity".to_string(), value)
    }

    // set order
    fn set_order(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "order".to_string(), value)
    }

    // set orphans
    fn set_orphans(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "orphans".to_string(), value)
    }

    // set outline
    fn set_outline(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline".to_string(), value)
    }

    // set outline-color
    fn set_outline_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline-color".to_string(), value)
    }

    // set outline-style
    fn set_outline_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline-style".to_string(), value)
    }

    // set outline-width
    fn set_outline_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline-width".to_string(), value)
    }

    // set outline-offset
    fn set_outline_offset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline-offset".to_string(), value)
    }

    // set overflow
    fn set_overflow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow".to_string(), value)
    }

    // set overflow-x
    fn set_overflow_x(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow-x".to_string(), value)
    }

    // set overflow-y
    fn set_overflow_y(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow-y".to_string(), value)
    }

    // set overflow-style
    fn set_overflow_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow-style".to_string(), value)
    }

    // set overflow-wrap
    fn set_overflow_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow-wrap".to_string(), value)
    }

    // set padding
    fn set_padding(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding".to_string(), value)
    }

    // set padding-bottom
    fn set_padding_bottom(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding-bottom".to_string(), value)
    }

    // set padding-left
    fn set_padding_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding-left".to_string(), value)
    }

    // set padding-right
    fn set_padding_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding-right".to_string(), value)
    }

    // set padding-top
    fn set_padding_top(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding-top".to_string(), value)
    }

    // set page
    fn set_page(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "page".to_string(), value)
    }

    // set page-break-after
    fn set_page_break_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "page-break-after".to_string(), value)
    }

    // set page-break-before
    fn set_page_break_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "page-break-before".to_string(), value)
    }

    // set page-break-inside
    fn set_page_break_inside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "page-break-inside".to_string(), value)
    }

    // set paint-order
    fn set_paint_order(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "paint-order".to_string(), value)
    }

    // set pause
    fn set_pause(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pause".to_string(), value)
    }

    // set pause-after
    fn set_pause_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pause-after".to_string(), value)
    }

    // set pause-before
    fn set_pause_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pause-before".to_string(), value)
    }

    // set perspective
    fn set_perspective(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "perspective".to_string(), value)
    }

    // set perspective-origin
    fn set_perspective_origin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "perspective-origin".to_string(), value)
    }

    // set pitch
    fn set_pitch(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pitch".to_string(), value)
    }

    // set pitch-range
    fn set_pitch_range(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pitch-range".to_string(), value)
    }

    // set play-during
    fn set_play_during(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "play-during".to_string(), value)
    }

    // set pointer-events
    fn set_pointer_events(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pointer-events".to_string(), value)
    }

    // set position
    fn set_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "position".to_string(), value)
    }

    // set quotes
    fn set_quotes(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "quotes".to_string(), value)
    }

    // set region-fragment
    fn set_region_fragment(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "region-fragment".to_string(), value)
    }

    // set resize
    fn set_resize(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "resize".to_string(), value)
    }

    // set rest
    fn set_rest(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "rest".to_string(), value)
    }

    // set rest-after
    fn set_rest_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "rest-after".to_string(), value)
    }

    // set rest-before
    fn set_rest_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "rest-before".to_string(), value)
    }

    // set richness
    fn set_richness(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "richness".to_string(), value)
    }

    // set right
    fn set_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "right".to_string(), value)
    }

    // set ruby-align
    fn set_ruby_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ruby-align".to_string(), value)
    }

    // set ruby-merge
    fn set_ruby_merge(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ruby-merge".to_string(), value)
    }

    // set ruby-position
    fn set_ruby_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ruby-position".to_string(), value)
    }

    // set scroll-behavior
    fn set_scroll_behavior(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-behavior".to_string(), value)
    }

    // set scroll-snap-coordinate
    fn set_scroll_snap_coordinate(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-coordinate".to_string(), value)
    }

    // set scroll-snap-destination
    fn set_scroll_snap_destination(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-destination".to_string(), value)
    }

    // set scroll-snap-points-x
    fn set_scroll_snap_points_x(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-points-x".to_string(), value)
    }

    // set scroll-snap-points-y
    fn set_scroll_snap_points_y(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-points-y".to_string(), value)
    }

    // set scroll-snap-type
    fn set_scroll_snap_type(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-type".to_string(), value)
    }

    // set shape-image-threshold
    fn set_shape_image_threshold(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-image-threshold".to_string(), value)
    }

    // set shape-inside
    fn set_shape_inside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-inside".to_string(), value)
    }

    // set shape-margin
    fn set_shape_margin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-margin".to_string(), value)
    }

    // set shape-outside
    fn set_shape_outside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-outside".to_string(), value)
    }

    // set shape-padding
    fn set_shape_padding(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-padding".to_string(), value)
    }

    // set shape-rendering
    fn set_shape_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-rendering".to_string(), value)
    }

    // set size
    fn set_size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "size".to_string(), value)
    }

    // set speak
    fn set_speak(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak".to_string(), value)
    }

    // set speak-as
    fn set_speak_as(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak-as".to_string(), value)
    }

    // set speak-header
    fn set_speak_header(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak-header".to_string(), value)
    }

    // set speak-numeral
    fn set_speak_numeral(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak-numeral".to_string(), value)
    }

    // set speak-punctuation
    fn set_speak_punctuation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak-punctuation".to_string(), value)
    }

    // set speech-rate
    fn set_speech_rate(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speech-rate".to_string(), value)
    }

    // set stop-color
    fn set_stop_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stop-color".to_string(), value)
    }

    // set stop-opacity
    fn set_stop_opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stop-opacity".to_string(), value)
    }

    // set stress
    fn set_stress(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stress".to_string(), value)
    }

    // set string-set
    fn set_string_set(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "string-set".to_string(), value)
    }

    // set stroke
    fn set_stroke(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke".to_string(), value)
    }

    // set stroke-dasharray
    fn set_stroke_dasharray(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-dasharray".to_string(), value)
    }

    // set stroke-dashoffset
    fn set_stroke_dashoffset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-dashoffset".to_string(), value)
    }

    // set stroke-linecap
    fn set_stroke_linecap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-linecap".to_string(), value)
    }

    // set stroke-linejoin
    fn set_stroke_linejoin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-linejoin".to_string(), value)
    }

    // set stroke-miterlimit
    fn set_stroke_miterlimit(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-miterlimit".to_string(), value)
    }

    // set stroke-opacity
    fn set_stroke_opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-opacity".to_string(), value)
    }

    // set stroke-width
    fn set_stroke_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-width".to_string(), value)
    }

    // set tab-size
    fn set_tab_size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "tab-size".to_string(), value)
    }

    // set table-layout
    fn set_table_layout(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "table-layout".to_string(), value)
    }

    // set text-align
    fn set_text_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-align".to_string(), value)
    }

    // set text-align-all
    fn set_text_align_all(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-align-all".to_string(), value)
    }

    // set text-align-last
    fn set_text_align_last(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-align-last".to_string(), value)
    }

    // set text-anchor
    fn set_text_anchor(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-anchor".to_string(), value)
    }

    // set text-combine-upright
    fn set_text_combine_upright(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-combine-upright".to_string(), value)
    }

    // set text-decoration
    fn set_text_decoration(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration".to_string(), value)
    }

    // set text-decoration-color
    fn set_text_decoration_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration-color".to_string(), value)
    }

    // set text-decoration-line
    fn set_text_decoration_line(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration-line".to_string(), value)
    }

    // set text-decoration-style
    fn set_text_decoration_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration-style".to_string(), value)
    }

    // set text-decoration-skip
    fn set_text_decoration_skip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration-skip".to_string(), value)
    }

    // set text-emphasis
    fn set_text_emphasis(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis".to_string(), value)
    }

    // set text-emphasis-color
    fn set_text_emphasis_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis-color".to_string(), value)
    }

    // set text-emphasis-style
    fn set_text_emphasis_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis-style".to_string(), value)
    }

    // set text-emphasis-position
    fn set_text_emphasis_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis-position".to_string(), value)
    }

    // set text-emphasis-skip
    fn set_text_emphasis_skip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis-skip".to_string(), value)
    }

    // set text-height
    fn set_text_height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-height".to_string(), value)
    }

    // set text-indent
    fn set_text_indent(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-indent".to_string(), value)
    }

    // set text-justify
    fn set_text_justify(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-justify".to_string(), value)
    }

    // set text-orientation
    fn set_text_orientation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-orientation".to_string(), value)
    }

    // set text-overflow
    fn set_text_overflow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-overflow".to_string(), value)
    }

    // set text-rendering
    fn set_text_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-rendering".to_string(), value)
    }

    // set text-shadow
    fn set_text_shadow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-shadow".to_string(), value)
    }

    // set text-size-adjust
    fn set_text_size_adjust(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-size-adjust".to_string(), value)
    }

    // set text-space-collapse
    fn set_text_space_collapse(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-space-collapse".to_string(), value)
    }

    // set text-spacing
    fn set_text_spacing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-spacing".to_string(), value)
    }

    // set text-transform
    fn set_text_transform(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-transform".to_string(), value)
    }

    // set text-underline-position
    fn set_text_underline_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-underline-position".to_string(), value)
    }

    // set text-wrap
    fn set_text_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-wrap".to_string(), value)
    }

    // set top
    fn set_top(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "top".to_string(), value)
    }

    // set touch-action
    fn set_touch_action(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "touch-action".to_string(), value)
    }

    // set transform
    fn set_transform(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transform".to_string(), value)
    }

    // set transform-box
    fn set_transform_box(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transform-box".to_string(), value)
    }

    // set transform-origin
    fn set_transform_origin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transform-origin".to_string(), value)
    }

    // set transform-style
    fn set_transform_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transform-style".to_string(), value)
    }

    // set transition
    fn set_transition(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition".to_string(), value)
    }

    // set transition-delay
    fn set_transition_delay(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition-delay".to_string(), value)
    }

    // set transition-duration
    fn set_transition_duration(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition-duration".to_string(), value)
    }

    // set transition-property
    fn set_transition_property(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition-property".to_string(), value)
    }

    // set unicode-bidi
    fn set_unicode_bidi(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "unicode-bidi".to_string(), value)
    }

    // set vector-effect
    fn set_vector_effect(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "vector-effect".to_string(), value)
    }

    // set vertical-align
    fn set_vertical_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "vertical-align".to_string(), value)
    }

    // set visibility
    fn set_visibility(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "visibility".to_string(), value)
    }

    // set voice-balance
    fn set_voice_balance(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-balance".to_string(), value)
    }

    // set voice-duration
    fn set_voice_duration(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-duration".to_string(), value)
    }

    // set voice-family
    fn set_voice_family(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-family".to_string(), value)
    }

    // set voice-pitch
    fn set_voice_pitch(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-pitch".to_string(), value)
    }

    // set voice-range
    fn set_voice_range(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-range".to_string(), value)
    }

    // set voice-rate
    fn set_voice_rate(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-rate".to_string(), value)
    }

    // set voice-stress
    fn set_voice_stress(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-stress".to_string(), value)
    }

    // set voice-volumn
    fn set_voice_volumn(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-volumn".to_string(), value)
    }

    // set volume
    fn set_volume(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "volume".to_string(), value)
    }

    // set white-space
    fn set_white_space(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "white-space".to_string(), value)
    }

    // set widows
    fn set_widows(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "widows".to_string(), value)
    }

    // set width
    fn set_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "width".to_string(), value)
    }

    // set will-change
    fn set_will_change(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "will-change".to_string(), value)
    }

    // set word-break
    fn set_word_break(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "word-break".to_string(), value)
    }

    // set word-spacing
    fn set_word_spacing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "word-spacing".to_string(), value)
    }

    // set word-wrap
    fn set_word_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "word-wrap".to_string(), value)
    }

    // set wrap-flow
    fn set_wrap_flow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "wrap-flow".to_string(), value)
    }

    // set wrap-through
    fn set_wrap_through(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "wrap-through".to_string(), value)
    }

    // set writing-mode
    fn set_writing_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "writing-mode".to_string(), value)
    }

    // set z-index
    fn set_z_index(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "z-index".to_string(), value)
    }

    // set transition-timing-function
    fn set_transition_timing_function(
        slf: PyRefMut<'_, Self>,
        value: String,
    ) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition-timing-function".to_string(), value)
    }
}
