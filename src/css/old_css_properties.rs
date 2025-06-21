use pyo3::prelude::*;
use std::collections::HashMap;

use crate::dom_components::StyleComponent;
use crate::utils::formaters;

/*/<·
#[pymethods]
impl StyleComponent {
    #[new]
    fn new() -> Self { // #.?
        StyleComponent {
            properties: HashMap::new(),
        }
    }
    fn __str__(&self) -> PyResult<String> { //#.?
        let entries = self.__simple_base_content();
        let fmt: Vec<String> = entries
            .iter()
            .map(|entry| format!("\t{}", entry))
            .collect();

        Ok(format!("{{\n{}\n}}", fmt.join("\n")))
    } // __str__

    fn as_class(&self, class_name: String) -> PyResult<String> { //<·str·>!
        let cls_str = self.__str__()?;

        Ok(
            format!(
                ".{} {}",
                class_name, cls_str
            )
        )
    } // __as_class__

    fn as_tag(&self, class_name: String) -> PyResult<String> { // <·str·>!
        let entries = self.__simple_base_content();
        let fmt: Vec<String> = entries
            .iter()
            .map(|entry| format!("\t\t{}", entry))
            .collect();

        let tag_str = format!(
            "<style>\n\t{} {{\n{}\n}}\n</style>\n",
            class_name,
            fmt.join("\n")
        );
        Ok(tag_str)
    } // __as_tag__

    fn inline(&self, use_attr: bool) -> PyResult<String> { // <·str·>!
        let entries = self.__simple_base_content();

        let prefix = "style=\"";

        if use_attr {
            return Ok(format!("{}{}\"", prefix, entries.join("")));
        }
        Ok(format!("{}", entries.join("")))
    } // inline

    // css props
    fn set_property(
        mut slf: PyRefMut<'_, Self>,
        name: String,
        value: String,
    ) -> PyRefMut<'_, Self> {
        let _ = slf
            .properties
            .insert(name, format!("{}", formaters::repr(&value)));
        slf
    }

    */
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

    fn elevation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "elevation".to_string(), value)
    }

    fn empty_cells(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "empty-cells".to_string(), value)
    }

    fn enable_background(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "enable-background".to_string(), value)
    }

    fn fill(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "fill".to_string(), value)
    }

    fn fill_opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "fill-opacity".to_string(), value)
    }

    fn fill_rule(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "fill-rule".to_string(), value)
    }

    fn filter(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "filter".to_string(), value)
    }

    fn float(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float".to_string(), value)
    }

    fn float_defer_column(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float-defer-column".to_string(), value)
    }

    fn float_defer_page(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float-defer-page".to_string(), value)
    }

    fn float_offset(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float-offset".to_string(), value)
    }

    fn float_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "float-wrap".to_string(), value)
    }

    fn flow_into(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flow-into".to_string(), value)
    }

    fn flow_from(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flow-from".to_string(), value)
    }

    fn flex(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex".to_string(), value)
    }

    fn flex_basis(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-basis".to_string(), value)
    }

    fn flex_grow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-grow".to_string(), value)
    }

    fn flex_shrink(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-shrink".to_string(), value)
    }

    fn flex_flow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-flow".to_string(), value)
    }

    fn flex_direction(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-direction".to_string(), value)
    }

    fn flex_wrap(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flex-wrap".to_string(), value)
    }

    fn flood_color(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flood-color".to_string(), value)
    }

    fn flood_opacity(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "flood-opacity".to_string(), value)
    }

    fn font(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font".to_string(), value)
    }

    fn font_family(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-family".to_string(), value)
    }

    fn font_size(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-size".to_string(), value)
    }

    fn font_stretch(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-stretch".to_string(), value)
    }

    fn font_style(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-style".to_string(), value)
    }

    fn font_weight(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-weight".to_string(), value)
    }

    fn font_feature_settings(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-feature-settings".to_string(), value)
    }

    fn font_kerning(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-kerning".to_string(), value)
    }

    fn font_language_override(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-language-override".to_string(), value)
    }

    fn font_size_adjust(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-size-adjust".to_string(), value)
    }

    fn font_synthesis(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-synthesis".to_string(), value)
    }

    fn font_variant(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant".to_string(), value)
    }

    fn font_variant_alternates(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-alternates".to_string(), value)
    }

    fn font_variant_caps(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-caps".to_string(), value)
    }

    fn font_variant_east_asian(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-east-asian".to_string(), value)
    }

    fn font_variant_ligatures(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-ligatures".to_string(), value)
    }

    fn font_variant_numeric(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-numeric".to_string(), value)
    }

    fn font_variant_position(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "font-variant-position".to_string(), value)
    }

    fn footnote_policy(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "footnote-policy".to_string(), value)
    }

    fn glyph_orientation_horizontal(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "glyph-orientation-horizontal".to_string(), value)
    }

    fn glyph_orientation_vertical(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "glyph-orientation-vertical".to_string(), value)
    }

    fn grid(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid".to_string(), value)
    }

    fn grid_auto_flow(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-auto-flow".to_string(), value)
    }

    fn grid_auto_columns(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-auto-columns".to_string(), value)
    }

    fn grid_auto_rows(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-auto-rows".to_string(), value)
    }

    fn grid_template(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-template".to_string(), value)
    }

    fn grid_template_areas(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-template-areas".to_string(), value)
    }

    fn grid_template_columns(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-template-columns".to_string(), value)
    }

    fn grid_template_rows(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-template-rows".to_string(), value)
    }

    fn grid_area(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-area".to_string(), value)
    }

    fn grid_column(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-column".to_string(), value)
    }

    fn grid_column_start(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-column-start".to_string(), value)
    }

    fn grid_column_end(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-column-end".to_string(), value)
    }

    fn grid_row(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-row".to_string(), value)
    }

    fn grid_row_start(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-row-start".to_string(), value)
    }

    fn grid_row_end(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "grid-row-end".to_string(), value)
    }

    fn hanging_punctuation(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hanging-punctuation".to_string(), value)
    }

    fn height(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "height".to_string(), value)
    }

    fn hyphenate_character(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-character".to_string(), value)
    }

    fn hyphenate_limit_chars(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-limit-chars".to_string(), value)
    }

    fn hyphenate_limit_last(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-limit-last".to_string(), value)
    }

    fn hyphenate_limit_lines(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-limit-lines".to_string(), value)
    }

    fn hyphenate_limit_zone(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphenate-limit-zone".to_string(), value)
    }

    fn hyphens(slf: PyRefMut<'_, Self>, value: String) -> PyRefMut<'_, Self> {
        Self::set_property(slf, "hyphens".to_string(), value)
    }

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
