use pyo3::prelude::*;

use crate::dom_components::StyleComponent;

//<·
#[pymethods]
impl StyleComponent {
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
}
