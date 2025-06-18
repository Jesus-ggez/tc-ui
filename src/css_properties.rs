use pyo3::prelude::*;
use std::collections::HashMap;

use crate::dom_components::StyleComponent;

//<·
fn repr(val: &str) -> String {
    let keywords = [];

    if keywords.contains(&val) {
        return val.to_string();
    }

    let is_measure = val.chars().next().map_or(false, |c| c.is_ascii_digit())
        && val.chars().any(|c| c.is_ascii_alphabetic());

    if is_measure {
        return val.to_string();
    }

    format!("\"{}\"", val)
}

#[pymethods]
impl StyleComponent {
    #[new]
    fn new() -> Self {
        StyleComponent {
            properties: HashMap::new(),
        }
    }
    fn set_property(
        mut slf: PyRefMut<'_, Self>,
        name: String,
        value: String,
    ) -> PyRefMut<'_, Self> {
        let _ = slf.properties.insert(name, format!("{}", repr(&value)));
        slf
    }

    fn __str__(&self) -> PyResult<String> {
        let mut entries = Vec::new();

        for (k, v) in &self.properties {
            entries.push(format!("\t{}: {};\n", k, v));
        }
        let self_str = format!("{{\n{}}}", entries.join(""));

        Ok(self_str)
    }

    // set align-content
    fn align_content(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "align-content".to_string(), value)
    }

    // set align-items
    fn align_items(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "align-items".to_string(), value)
    }

    // set align-self
    fn align_self(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "align-self".to_string(), value)
    }

    // set alignment-adjust
    fn alignment_adjust(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "alignment-adjust".to_string(), value)
    }

    // set alignment-baseline
    fn alignment_baseline(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "alignment-baseline".to_string(), value)
    }

    // set all
    fn all(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "all".to_string(), value)
    }

    // set alt
    fn alt(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "alt".to_string(), value)
    }

    // set animation
    fn animation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation".to_string(), value)
    }

    // set animation-delay
    fn animation_delay(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-delay".to_string(), value)
    }

    // set animation-direction
    fn animation_direction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-direction".to_string(), value)
    }

    // set animation-duration
    fn animation_duration(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-duration".to_string(), value)
    }

    // set animation-fill-mode
    fn animation_fill_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-fill-mode".to_string(), value)
    }

    // set animation-iteration-count
    fn animation_iteration_count(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-iteration-count".to_string(), value)
    }

    // set animation-name
    fn animation_name(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-name".to_string(), value)
    }

    // set animation-play-state
    fn animation_play_state(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-play-state".to_string(), value)
    }

    // set animation-timing-function
    fn animation_timing_function(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "animation-timing-function".to_string(), value)
    }

    // set azimuth
    fn azimuth(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "azimuth".to_string(), value)
    }

    // set backface-visibility
    fn backface_visibility(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "backface-visibility".to_string(), value)
    }

    // set background
    fn background(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background".to_string(), value)
    }

    // set background-attachment
    fn background_attachment(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-attachment".to_string(), value)
    }

    // set background-clip
    fn background_clip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-clip".to_string(), value)
    }

    // set background-color
    fn background_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-color".to_string(), value)
    }

    // set background-image
    fn background_image(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-image".to_string(), value)
    }

    // set background-origin
    fn background_origin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-origin".to_string(), value)
    }

    // set background-position
    fn background_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-position".to_string(), value)
    }

    // set background-repeat
    fn background_repeat(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-repeat".to_string(), value)
    }

    // set background-size
    fn background_size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-size".to_string(), value)
    }

    // set background-blend-mode
    fn background_blend_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "background-blend-mode".to_string(), value)
    }

    // set baseline-shift
    fn baseline_shift(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "baseline-shift".to_string(), value)
    }

    // set bleed
    fn bleed(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bleed".to_string(), value)
    }

    // set bookmark-label
    fn bookmark_label(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bookmark-label".to_string(), value)
    }

    // set bookmark-level
    fn bookmark_level(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bookmark-level".to_string(), value)
    }

    // set bookmark-state
    fn bookmark_state(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bookmark-state".to_string(), value)
    }

    // set border
    fn border(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border".to_string(), value)
    }

    // set border-color
    fn border_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-color".to_string(), value)
    }

    // set border-style
    fn border_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-style".to_string(), value)
    }

    // set border-width
    fn border_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-width".to_string(), value)
    }

    // set border-bottom
    fn border_bottom(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom".to_string(), value)
    }

    // set border-bottom-color
    fn border_bottom_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-color".to_string(), value)
    }

    // set border-bottom-style
    fn border_bottom_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-style".to_string(), value)
    }

    // set border-bottom-width
    fn border_bottom_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-width".to_string(), value)
    }

    // set border-left
    fn border_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-left".to_string(), value)
    }

    // set border-left-color
    fn border_left_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-left-color".to_string(), value)
    }

    // set border-left-style
    fn border_left_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-left-style".to_string(), value)
    }

    // set border-left-width
    fn border_left_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-left-width".to_string(), value)
    }

    // set border-right
    fn border_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-right".to_string(), value)
    }

    // set border-right-color
    fn border_right_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-right-color".to_string(), value)
    }

    // set border-right-style
    fn border_right_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-right-style".to_string(), value)
    }

    // set border-right-width
    fn border_right_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-right-width".to_string(), value)
    }

    // set border-top
    fn border_top(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top".to_string(), value)
    }

    // set border-top-color
    fn border_top_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-color".to_string(), value)
    }

    // set border-top-style
    fn border_top_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-style".to_string(), value)
    }

    // set border-top-width
    fn border_top_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-width".to_string(), value)
    }

    // set border-collapse
    fn border_collapse(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-collapse".to_string(), value)
    }

    // set border-image
    fn border_image(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image".to_string(), value)
    }

    // set border-image-outset
    fn border_image_outset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-outset".to_string(), value)
    }

    // set border-image-repeat
    fn border_image_repeat(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-repeat".to_string(), value)
    }

    // set border-image-slice
    fn border_image_slice(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-slice".to_string(), value)
    }

    // set border-image-source
    fn border_image_source(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-source".to_string(), value)
    }

    // set border-image-width
    fn border_image_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-image-width".to_string(), value)
    }

    // set border-radius
    fn border_radius(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-radius".to_string(), value)
    }

    // set border-bottom-left-radius
    fn border_bottom_left_radius(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-left-radius".to_string(), value)
    }

    // set border-bottom-right-radius
    fn border_bottom_right_radius(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-bottom-right-radius".to_string(), value)
    }

    // set border-top-left-radius
    fn border_top_left_radius(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-left-radius".to_string(), value)
    }

    // set border-top-right-radius
    fn border_top_right_radius(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-top-right-radius".to_string(), value)
    }

    // set border-spacing
    fn border_spacing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "border-spacing".to_string(), value)
    }

    // set bottom
    fn bottom(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "bottom".to_string(), value)
    }

    // set box-decoration-break
    fn box_decoration_break(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "box-decoration-break".to_string(), value)
    }

    // set box-shadow
    fn box_shadow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "box-shadow".to_string(), value)
    }

    // set box-sizing
    fn box_sizing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "box-sizing".to_string(), value)
    }

    // set box-snap
    fn box_snap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "box-snap".to_string(), value)
    }

    // set break-after
    fn break_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "break-after".to_string(), value)
    }

    // set break-before
    fn break_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "break-before".to_string(), value)
    }

    // set break-inside
    fn break_inside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "break-inside".to_string(), value)
    }

    // set buffered-rendering
    fn buffered_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "buffered-rendering".to_string(), value)
    }

    // set caption-side
    fn caption_side(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "caption-side".to_string(), value)
    }

    // set clear
    fn clear(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clear".to_string(), value)
    }

    // set clear-side
    fn clear_side(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clear-side".to_string(), value)
    }

    // set clip
    fn clip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clip".to_string(), value)
    }

    // set clip-path
    fn clip_path(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clip-path".to_string(), value)
    }

    // set clip-rule
    fn clip_rule(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "clip-rule".to_string(), value)
    }

    // set color
    fn color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color".to_string(), value)
    }

    // set color-adjust
    fn color_adjust(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-adjust".to_string(), value)
    }

    // set color-correction
    fn color_correction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-correction".to_string(), value)
    }

    // set color-interpolation
    fn color_interpolation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-interpolation".to_string(), value)
    }

    // set color-interpolation-filters
    fn color_interpolation_filters(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-interpolation-filters".to_string(), value)
    }

    // set color-profile
    fn color_profile(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-profile".to_string(), value)
    }

    // set color-rendering
    fn color_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "color-rendering".to_string(), value)
    }

    // set column-fill
    fn column_fill(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-fill".to_string(), value)
    }

    // set column-gap
    fn column_gap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-gap".to_string(), value)
    }

    // set column-rule
    fn column_rule(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-rule".to_string(), value)
    }

    // set column-rule-color
    fn column_rule_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-rule-color".to_string(), value)
    }

    // set column-rule-style
    fn column_rule_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-rule-style".to_string(), value)
    }

    // set column-rule-width
    fn column_rule_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-rule-width".to_string(), value)
    }

    // set column-span
    fn column_span(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-span".to_string(), value)
    }

    // set columns
    fn columns(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "columns".to_string(), value)
    }

    // set column-count
    fn column_count(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-count".to_string(), value)
    }

    // set column-width
    fn column_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "column-width".to_string(), value)
    }

    // set contain
    fn contain(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "contain".to_string(), value)
    }

    // set content
    fn content(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "content".to_string(), value)
    }

    // set counter-increment
    fn counter_increment(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "counter-increment".to_string(), value)
    }

    // set counter-reset
    fn counter_reset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "counter-reset".to_string(), value)
    }

    // set counter-set
    fn counter_set(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "counter-set".to_string(), value)
    }

    // set cue
    fn cue(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "cue".to_string(), value)
    }

    // set cue-after
    fn cue_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "cue-after".to_string(), value)
    }

    // set cue-before
    fn cue_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "cue-before".to_string(), value)
    }

    // set cursor
    fn cursor(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "cursor".to_string(), value)
    }

    // set direction
    fn direction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "direction".to_string(), value)
    }

    // set display
    fn display(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display".to_string(), value)
    }

    // set display-inside
    fn display_inside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display-inside".to_string(), value)
    }

    // set display-outside
    fn display_outside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display-outside".to_string(), value)
    }

    // set display-extras
    fn display_extras(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display-extras".to_string(), value)
    }

    // set display-box
    fn display_box(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "display-box".to_string(), value)
    }

    // set dominant-baseline
    fn dominant_baseline(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "dominant-baseline".to_string(), value)
    }

    // set elevation
    fn elevation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "elevation".to_string(), value)
    }

    // set empty-cells
    fn empty_cells(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "empty-cells".to_string(), value)
    }

    // set enable-background
    fn enable_background(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "enable-background".to_string(), value)
    }

    // set fill
    fn fill(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "fill".to_string(), value)
    }

    // set fill-opacity
    fn fill_opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "fill-opacity".to_string(), value)
    }

    // set fill-rule
    fn fill_rule(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "fill-rule".to_string(), value)
    }

    // set filter
    fn filter(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "filter".to_string(), value)
    }

    // set float
    fn float(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float".to_string(), value)
    }

    // set float-defer-column
    fn float_defer_column(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float-defer-column".to_string(), value)
    }

    // set float-defer-page
    fn float_defer_page(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float-defer-page".to_string(), value)
    }

    // set float-offset
    fn float_offset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float-offset".to_string(), value)
    }

    // set float-wrap
    fn float_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float-wrap".to_string(), value)
    }

    // set flow-into
    fn flow_into(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flow-into".to_string(), value)
    }

    // set flow-from
    fn flow_from(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flow-from".to_string(), value)
    }

    // set flex
    fn flex(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex".to_string(), value)
    }

    // set flex-basis
    fn flex_basis(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-basis".to_string(), value)
    }

    // set flex-grow
    fn flex_grow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-grow".to_string(), value)
    }

    // set flex-shrink
    fn flex_shrink(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-shrink".to_string(), value)
    }

    // set flex-flow
    fn flex_flow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-flow".to_string(), value)
    }

    // set flex-direction
    fn flex_direction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-direction".to_string(), value)
    }

    // set flex-wrap
    fn flex_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-wrap".to_string(), value)
    }

    // set flood-color
    fn flood_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flood-color".to_string(), value)
    }

    // set flood-opacity
    fn flood_opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flood-opacity".to_string(), value)
    }

    // set font
    fn font(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font".to_string(), value)
    }

    // set font-family
    fn font_family(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-family".to_string(), value)
    }

    // set font-size
    fn font_size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-size".to_string(), value)
    }

    // set font-stretch
    fn font_stretch(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-stretch".to_string(), value)
    }

    // set font-style
    fn font_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-style".to_string(), value)
    }

    // set font-weight
    fn font_weight(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-weight".to_string(), value)
    }

    // set font-feature-settings
    fn font_feature_settings(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-feature-settings".to_string(), value)
    }

    // set font-kerning
    fn font_kerning(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-kerning".to_string(), value)
    }

    // set font-language-override
    fn font_language_override(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-language-override".to_string(), value)
    }

    // set font-size-adjust
    fn font_size_adjust(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-size-adjust".to_string(), value)
    }

    // set font-synthesis
    fn font_synthesis(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-synthesis".to_string(), value)
    }

    // set font-variant
    fn font_variant(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant".to_string(), value)
    }

    // set font-variant-alternates
    fn font_variant_alternates(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-alternates".to_string(), value)
    }

    // set font-variant-caps
    fn font_variant_caps(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-caps".to_string(), value)
    }

    // set font-variant-east-asian
    fn font_variant_east_asian(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-east-asian".to_string(), value)
    }

    // set font-variant-ligatures
    fn font_variant_ligatures(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-ligatures".to_string(), value)
    }

    // set font-variant-numeric
    fn font_variant_numeric(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-numeric".to_string(), value)
    }

    // set font-variant-position
    fn font_variant_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-position".to_string(), value)
    }

    // set footnote-policy
    fn footnote_policy(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "footnote-policy".to_string(), value)
    }

    // set glyph-orientation-horizontal
    fn glyph_orientation_horizontal(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "glyph-orientation-horizontal".to_string(), value)
    }

    // set glyph-orientation-vertical
    fn glyph_orientation_vertical(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "glyph-orientation-vertical".to_string(), value)
    }

    // set grid
    fn grid(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid".to_string(), value)
    }

    // set grid-auto-flow
    fn grid_auto_flow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-auto-flow".to_string(), value)
    }

    // set grid-auto-columns
    fn grid_auto_columns(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-auto-columns".to_string(), value)
    }

    // set grid-auto-rows
    fn grid_auto_rows(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-auto-rows".to_string(), value)
    }

    // set grid-template
    fn grid_template(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-template".to_string(), value)
    }

    // set grid-template-areas
    fn grid_template_areas(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-template-areas".to_string(), value)
    }

    // set grid-template-columns
    fn grid_template_columns(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-template-columns".to_string(), value)
    }

    // set grid-template-rows
    fn grid_template_rows(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-template-rows".to_string(), value)
    }

    // set grid-area
    fn grid_area(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-area".to_string(), value)
    }

    // set grid-column
    fn grid_column(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-column".to_string(), value)
    }

    // set grid-column-start
    fn grid_column_start(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-column-start".to_string(), value)
    }

    // set grid-column-end
    fn grid_column_end(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-column-end".to_string(), value)
    }

    // set grid-row
    fn grid_row(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-row".to_string(), value)
    }

    // set grid-row-start
    fn grid_row_start(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-row-start".to_string(), value)
    }

    // set grid-row-end
    fn grid_row_end(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-row-end".to_string(), value)
    }

    // set hanging-punctuation
    fn hanging_punctuation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hanging-punctuation".to_string(), value)
    }

    // set height
    fn height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "height".to_string(), value)
    }

    // set hyphenate-character
    fn hyphenate_character(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-character".to_string(), value)
    }

    // set hyphenate-limit-chars
    fn hyphenate_limit_chars(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-limit-chars".to_string(), value)
    }

    // set hyphenate-limit-last
    fn hyphenate_limit_last(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-limit-last".to_string(), value)
    }

    // set hyphenate-limit-lines
    fn hyphenate_limit_lines(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-limit-lines".to_string(), value)
    }

    // set hyphenate-limit-zone
    fn hyphenate_limit_zone(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-limit-zone".to_string(), value)
    }

    // set hyphens
    fn hyphens(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphens".to_string(), value)
    }

    // set icon
    fn icon(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "icon".to_string(), value)
    }

    // set image-orientation
    fn image_orientation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "image-orientation".to_string(), value)
    }

    // set image-resolution
    fn image_resolution(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "image-resolution".to_string(), value)
    }

    // set image-rendering
    fn image_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "image-rendering".to_string(), value)
    }

    // set ime
    fn ime(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime".to_string(), value)
    }

    // set ime-align
    fn ime_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime-align".to_string(), value)
    }

    // set ime-mode
    fn ime_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime-mode".to_string(), value)
    }

    // set ime-offset
    fn ime_offset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime-offset".to_string(), value)
    }

    // set ime-width
    fn ime_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ime-width".to_string(), value)
    }

    // set initial-letters
    fn initial_letters(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "initial-letters".to_string(), value)
    }

    // set inline-box-align
    fn inline_box_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "inline-box-align".to_string(), value)
    }

    // set isolation
    fn isolation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "isolation".to_string(), value)
    }

    // set justify-content
    fn justify_content(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "justify-content".to_string(), value)
    }

    // set justify-items
    fn justify_items(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "justify-items".to_string(), value)
    }

    // set justify-self
    fn justify_self(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "justify-self".to_string(), value)
    }

    // set kerning
    fn kerning(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "kerning".to_string(), value)
    }

    // set left
    fn left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "left".to_string(), value)
    }

    // set letter-spacing
    fn letter_spacing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "letter-spacing".to_string(), value)
    }

    // set lighting-color
    fn lighting_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "lighting-color".to_string(), value)
    }

    // set line-box-contain
    fn line_box_contain(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-box-contain".to_string(), value)
    }

    // set line-break
    fn line_break(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-break".to_string(), value)
    }

    // set line-grid
    fn line_grid(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-grid".to_string(), value)
    }

    // set line-height
    fn line_height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-height".to_string(), value)
    }

    // set line-slack
    fn line_slack(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-slack".to_string(), value)
    }

    // set line-snap
    fn line_snap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "line-snap".to_string(), value)
    }

    // set list-style
    fn list_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "list-style".to_string(), value)
    }

    // set list-style-image
    fn list_style_image(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "list-style-image".to_string(), value)
    }

    // set list-style-position
    fn list_style_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "list-style-position".to_string(), value)
    }

    // set list-style-type
    fn list_style_type(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "list-style-type".to_string(), value)
    }

    // set margin
    fn margin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin".to_string(), value)
    }

    // set margin-bottom
    fn margin_bottom(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin-bottom".to_string(), value)
    }

    // set margin-left
    fn margin_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin-left".to_string(), value)
    }

    // set margin-right
    fn margin_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin-right".to_string(), value)
    }

    // set margin-top
    fn margin_top(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "margin-top".to_string(), value)
    }

    // set marker
    fn marker(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker".to_string(), value)
    }

    // set marker-end
    fn marker_end(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-end".to_string(), value)
    }

    // set marker-mid
    fn marker_mid(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-mid".to_string(), value)
    }

    // set marker-pattern
    fn marker_pattern(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-pattern".to_string(), value)
    }

    // set marker-segment
    fn marker_segment(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-segment".to_string(), value)
    }

    // set marker-start
    fn marker_start(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-start".to_string(), value)
    }

    // set marker-knockout-left
    fn marker_knockout_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-knockout-left".to_string(), value)
    }

    // set marker-knockout-right
    fn marker_knockout_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-knockout-right".to_string(), value)
    }

    // set marker-side
    fn marker_side(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marker-side".to_string(), value)
    }

    // set marks
    fn marks(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marks".to_string(), value)
    }

    // set marquee-direction
    fn marquee_direction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marquee-direction".to_string(), value)
    }

    // set marquee-play-count
    fn marquee_play_count(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marquee-play-count".to_string(), value)
    }

    // set marquee-speed
    fn marquee_speed(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marquee-speed".to_string(), value)
    }

    // set marquee-style
    fn marquee_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "marquee-style".to_string(), value)
    }

    // set mask
    fn mask(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask".to_string(), value)
    }

    // set mask-image
    fn mask_image(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-image".to_string(), value)
    }

    // set mask-repeat
    fn mask_repeat(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-repeat".to_string(), value)
    }

    // set mask-position
    fn mask_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-position".to_string(), value)
    }

    // set mask-clip
    fn mask_clip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-clip".to_string(), value)
    }

    // set mask-origin
    fn mask_origin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-origin".to_string(), value)
    }

    // set mask-size
    fn mask_size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-size".to_string(), value)
    }

    // set mask-box
    fn mask_box(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box".to_string(), value)
    }

    // set mask-box-outset
    fn mask_box_outset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-outset".to_string(), value)
    }

    // set mask-box-repeat
    fn mask_box_repeat(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-repeat".to_string(), value)
    }

    // set mask-box-slice
    fn mask_box_slice(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-slice".to_string(), value)
    }

    // set mask-box-source
    fn mask_box_source(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-source".to_string(), value)
    }

    // set mask-box-width
    fn mask_box_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-box-width".to_string(), value)
    }

    // set mask-type
    fn mask_type(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mask-type".to_string(), value)
    }

    // set max-height
    fn max_height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "max-height".to_string(), value)
    }

    // set max-lines
    fn max_lines(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "max-lines".to_string(), value)
    }

    // set max-width
    fn max_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "max-width".to_string(), value)
    }

    // set min-height
    fn min_height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "min-height".to_string(), value)
    }

    // set min-width
    fn min_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "min-width".to_string(), value)
    }

    // set mix-blend-mode
    fn mix_blend_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "mix-blend-mode".to_string(), value)
    }

    // set nav-down
    fn nav_down(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-down".to_string(), value)
    }

    // set nav-index
    fn nav_index(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-index".to_string(), value)
    }

    // set nav-left
    fn nav_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-left".to_string(), value)
    }

    // set nav-right
    fn nav_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-right".to_string(), value)
    }

    // set nav-up
    fn nav_up(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "nav-up".to_string(), value)
    }

    // set object-fit
    fn object_fit(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "object-fit".to_string(), value)
    }

    // set object-position
    fn object_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "object-position".to_string(), value)
    }

    // set offset-after
    fn offafter(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "offset-after".to_string(), value)
    }

    // set offset-before
    fn offbefore(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "offset-before".to_string(), value)
    }

    // set offset-end
    fn offend(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "offset-end".to_string(), value)
    }

    // set offset-start
    fn offstart(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "offset-start".to_string(), value)
    }

    // set opacity
    fn opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "opacity".to_string(), value)
    }

    // set order
    fn order(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "order".to_string(), value)
    }

    // set orphans
    fn orphans(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "orphans".to_string(), value)
    }

    // set outline
    fn outline(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline".to_string(), value)
    }

    // set outline-color
    fn outline_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline-color".to_string(), value)
    }

    // set outline-style
    fn outline_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline-style".to_string(), value)
    }

    // set outline-width
    fn outline_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline-width".to_string(), value)
    }

    // set outline-offset
    fn outline_offset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "outline-offset".to_string(), value)
    }

    // set overflow
    fn overflow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow".to_string(), value)
    }

    // set overflow-x
    fn overflow_x(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow-x".to_string(), value)
    }

    // set overflow-y
    fn overflow_y(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow-y".to_string(), value)
    }

    // set overflow-style
    fn overflow_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow-style".to_string(), value)
    }

    // set overflow-wrap
    fn overflow_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "overflow-wrap".to_string(), value)
    }

    // set padding
    fn padding(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding".to_string(), value)
    }

    // set padding-bottom
    fn padding_bottom(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding-bottom".to_string(), value)
    }

    // set padding-left
    fn padding_left(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding-left".to_string(), value)
    }

    // set padding-right
    fn padding_right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding-right".to_string(), value)
    }

    // set padding-top
    fn padding_top(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "padding-top".to_string(), value)
    }

    // set page
    fn page(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "page".to_string(), value)
    }

    // set page-break-after
    fn page_break_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "page-break-after".to_string(), value)
    }

    // set page-break-before
    fn page_break_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "page-break-before".to_string(), value)
    }

    // set page-break-inside
    fn page_break_inside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "page-break-inside".to_string(), value)
    }

    // set paint-order
    fn paint_order(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "paint-order".to_string(), value)
    }

    // set pause
    fn pause(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pause".to_string(), value)
    }

    // set pause-after
    fn pause_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pause-after".to_string(), value)
    }

    // set pause-before
    fn pause_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pause-before".to_string(), value)
    }

    // set perspective
    fn perspective(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "perspective".to_string(), value)
    }

    // set perspective-origin
    fn perspective_origin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "perspective-origin".to_string(), value)
    }

    // set pitch
    fn pitch(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pitch".to_string(), value)
    }

    // set pitch-range
    fn pitch_range(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pitch-range".to_string(), value)
    }

    // set play-during
    fn play_during(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "play-during".to_string(), value)
    }

    // set pointer-events
    fn pointer_events(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "pointer-events".to_string(), value)
    }

    // set position
    fn position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "position".to_string(), value)
    }

    // set quotes
    fn quotes(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "quotes".to_string(), value)
    }

    // set region-fragment
    fn region_fragment(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "region-fragment".to_string(), value)
    }

    // set resize
    fn resize(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "resize".to_string(), value)
    }

    // set rest
    fn rest(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "rest".to_string(), value)
    }

    // set rest-after
    fn rest_after(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "rest-after".to_string(), value)
    }

    // set rest-before
    fn rest_before(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "rest-before".to_string(), value)
    }

    // set richness
    fn richness(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "richness".to_string(), value)
    }

    // set right
    fn right(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "right".to_string(), value)
    }

    // set ruby-align
    fn ruby_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ruby-align".to_string(), value)
    }

    // set ruby-merge
    fn ruby_merge(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ruby-merge".to_string(), value)
    }

    // set ruby-position
    fn ruby_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "ruby-position".to_string(), value)
    }

    // set scroll-behavior
    fn scroll_behavior(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-behavior".to_string(), value)
    }

    // set scroll-snap-coordinate
    fn scroll_snap_coordinate(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-coordinate".to_string(), value)
    }

    // set scroll-snap-destination
    fn scroll_snap_destination(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-destination".to_string(), value)
    }

    // set scroll-snap-points-x
    fn scroll_snap_points_x(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-points-x".to_string(), value)
    }

    // set scroll-snap-points-y
    fn scroll_snap_points_y(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-points-y".to_string(), value)
    }

    // set scroll-snap-type
    fn scroll_snap_type(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "scroll-snap-type".to_string(), value)
    }

    // set shape-image-threshold
    fn shape_image_threshold(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-image-threshold".to_string(), value)
    }

    // set shape-inside
    fn shape_inside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-inside".to_string(), value)
    }

    // set shape-margin
    fn shape_margin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-margin".to_string(), value)
    }

    // set shape-outside
    fn shape_outside(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-outside".to_string(), value)
    }

    // set shape-padding
    fn shape_padding(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-padding".to_string(), value)
    }

    // set shape-rendering
    fn shape_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "shape-rendering".to_string(), value)
    }

    // set size
    fn size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "size".to_string(), value)
    }

    // set speak
    fn speak(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak".to_string(), value)
    }

    // set speak-as
    fn speak_as(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak-as".to_string(), value)
    }

    // set speak-header
    fn speak_header(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak-header".to_string(), value)
    }

    // set speak-numeral
    fn speak_numeral(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak-numeral".to_string(), value)
    }

    // set speak-punctuation
    fn speak_punctuation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speak-punctuation".to_string(), value)
    }

    // set speech-rate
    fn speech_rate(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "speech-rate".to_string(), value)
    }

    // set stop-color
    fn stop_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stop-color".to_string(), value)
    }

    // set stop-opacity
    fn stop_opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stop-opacity".to_string(), value)
    }

    // set stress
    fn stress(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stress".to_string(), value)
    }

    // set string-set
    fn string_set(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "string-set".to_string(), value)
    }

    // set stroke
    fn stroke(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke".to_string(), value)
    }

    // set stroke-dasharray
    fn stroke_dasharray(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-dasharray".to_string(), value)
    }

    // set stroke-dashoffset
    fn stroke_dashoffset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-dashoffset".to_string(), value)
    }

    // set stroke-linecap
    fn stroke_linecap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-linecap".to_string(), value)
    }

    // set stroke-linejoin
    fn stroke_linejoin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-linejoin".to_string(), value)
    }

    // set stroke-miterlimit
    fn stroke_miterlimit(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-miterlimit".to_string(), value)
    }

    // set stroke-opacity
    fn stroke_opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-opacity".to_string(), value)
    }

    // set stroke-width
    fn stroke_width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "stroke-width".to_string(), value)
    }

    // set tab-size
    fn tab_size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "tab-size".to_string(), value)
    }

    // set table-layout
    fn table_layout(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "table-layout".to_string(), value)
    }

    // set text-align
    fn text_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-align".to_string(), value)
    }

    // set text-align-all
    fn text_align_all(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-align-all".to_string(), value)
    }

    // set text-align-last
    fn text_align_last(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-align-last".to_string(), value)
    }

    // set text-anchor
    fn text_anchor(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-anchor".to_string(), value)
    }

    // set text-combine-upright
    fn text_combine_upright(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-combine-upright".to_string(), value)
    }

    // set text-decoration
    fn text_decoration(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration".to_string(), value)
    }

    // set text-decoration-color
    fn text_decoration_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration-color".to_string(), value)
    }

    // set text-decoration-line
    fn text_decoration_line(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration-line".to_string(), value)
    }

    // set text-decoration-style
    fn text_decoration_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration-style".to_string(), value)
    }

    // set text-decoration-skip
    fn text_decoration_skip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-decoration-skip".to_string(), value)
    }

    // set text-emphasis
    fn text_emphasis(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis".to_string(), value)
    }

    // set text-emphasis-color
    fn text_emphasis_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis-color".to_string(), value)
    }

    // set text-emphasis-style
    fn text_emphasis_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis-style".to_string(), value)
    }

    // set text-emphasis-position
    fn text_emphasis_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis-position".to_string(), value)
    }

    // set text-emphasis-skip
    fn text_emphasis_skip(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-emphasis-skip".to_string(), value)
    }

    // set text-height
    fn text_height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-height".to_string(), value)
    }

    // set text-indent
    fn text_indent(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-indent".to_string(), value)
    }

    // set text-justify
    fn text_justify(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-justify".to_string(), value)
    }

    // set text-orientation
    fn text_orientation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-orientation".to_string(), value)
    }

    // set text-overflow
    fn text_overflow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-overflow".to_string(), value)
    }

    // set text-rendering
    fn text_rendering(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-rendering".to_string(), value)
    }

    // set text-shadow
    fn text_shadow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-shadow".to_string(), value)
    }

    // set text-size-adjust
    fn text_size_adjust(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-size-adjust".to_string(), value)
    }

    // set text-space-collapse
    fn text_space_collapse(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-space-collapse".to_string(), value)
    }

    // set text-spacing
    fn text_spacing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-spacing".to_string(), value)
    }

    // set text-transform
    fn text_transform(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-transform".to_string(), value)
    }

    // set text-underline-position
    fn text_underline_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-underline-position".to_string(), value)
    }

    // set text-wrap
    fn text_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "text-wrap".to_string(), value)
    }

    // set top
    fn top(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "top".to_string(), value)
    }

    // set touch-action
    fn touch_action(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "touch-action".to_string(), value)
    }

    // set transform
    fn transform(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transform".to_string(), value)
    }

    // set transform-box
    fn transform_box(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transform-box".to_string(), value)
    }

    // set transform-origin
    fn transform_origin(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transform-origin".to_string(), value)
    }

    // set transform-style
    fn transform_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transform-style".to_string(), value)
    }

    // set transition
    fn transition(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition".to_string(), value)
    }

    // set transition-delay
    fn transition_delay(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition-delay".to_string(), value)
    }

    // set transition-duration
    fn transition_duration(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition-duration".to_string(), value)
    }

    // set transition-set_property
    fn transition_set_property(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition-set_property".to_string(), value)
    }

    // set unicode-bidi
    fn unicode_bidi(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "unicode-bidi".to_string(), value)
    }

    // set vector-effect
    fn vector_effect(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "vector-effect".to_string(), value)
    }

    // set vertical-align
    fn vertical_align(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "vertical-align".to_string(), value)
    }

    // set visibility
    fn visibility(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "visibility".to_string(), value)
    }

    // set voice-balance
    fn voice_balance(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-balance".to_string(), value)
    }

    // set voice-duration
    fn voice_duration(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-duration".to_string(), value)
    }

    // set voice-family
    fn voice_family(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-family".to_string(), value)
    }

    // set voice-pitch
    fn voice_pitch(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-pitch".to_string(), value)
    }

    // set voice-range
    fn voice_range(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-range".to_string(), value)
    }

    // set voice-rate
    fn voice_rate(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-rate".to_string(), value)
    }

    // set voice-stress
    fn voice_stress(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-stress".to_string(), value)
    }

    // set voice-volumn
    fn voice_volumn(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "voice-volumn".to_string(), value)
    }

    // set volume
    fn volume(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "volume".to_string(), value)
    }

    // set white-space
    fn white_space(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "white-space".to_string(), value)
    }

    // set widows
    fn widows(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "widows".to_string(), value)
    }

    // set width
    fn width(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "width".to_string(), value)
    }

    // set will-change
    fn will_change(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "will-change".to_string(), value)
    }

    // set word-break
    fn word_break(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "word-break".to_string(), value)
    }

    // set word-spacing
    fn word_spacing(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "word-spacing".to_string(), value)
    }

    // set word-wrap
    fn word_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "word-wrap".to_string(), value)
    }

    // set wrap-flow
    fn wrap_flow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "wrap-flow".to_string(), value)
    }

    // set wrap-through
    fn wrap_through(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "wrap-through".to_string(), value)
    }

    // set writing-mode
    fn writing_mode(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "writing-mode".to_string(), value)
    }

    // set z-index
    fn z_index(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "z-index".to_string(), value)
    }

    // set transition-timing-function
    fn transition_timing_function(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "transition-timing-function".to_string(), value)
    }
}
