// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/geo_line_strings.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../component_batch.hpp"
#include "../components/color.hpp"
#include "../components/geo_line_string.hpp"
#include "../components/radius.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: Geospatial line strings with positions expressed in EPSG:4326 altitude and longitude, and optional colors and radii.
    ///
    /// **Note**: Geospatial entities are experimental.
    ///
    /// ## Example
    ///
    /// ### Log a geospatial line string
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_geo_line_strings");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     rerun::Collection<rerun::DVec2D> line_string = {
    ///         {41.0000, -109.0452},
    ///         {41.0000, -102.0415},
    ///         {36.9931, -102.0415},
    ///         {36.9931, -109.0452},
    ///         {41.0000, -109.0452}};
    ///
    ///     rec.log(
    ///         "colorado",
    ///         rerun::GeoLineStrings({line_string})
    ///             .with_radii(rerun::Radius::ui_points(2.0f))
    ///             .with_colors(rerun::Color(0, 0, 255))
    ///     );
    /// }
    /// ```
    struct GeoLineStrings {
        /// The lines strings, expressed in EPSG:4326 coordinates.
        Collection<rerun::components::GeoLineString> line_strings;

        /// Optional radii for the line strings.
        std::optional<Collection<rerun::components::Radius>> radii;

        /// Optional colors for the linestrings.
        std::optional<Collection<rerun::components::Color>> colors;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.GeoLineStringsIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        GeoLineStrings() = default;
        GeoLineStrings(GeoLineStrings&& other) = default;

        explicit GeoLineStrings(Collection<rerun::components::GeoLineString> _line_strings)
            : line_strings(std::move(_line_strings)) {}

        /// Optional radii for the line strings.
        GeoLineStrings with_radii(Collection<rerun::components::Radius> _radii) && {
            radii = std::move(_radii);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional colors for the linestrings.
        GeoLineStrings with_colors(Collection<rerun::components::Color> _colors) && {
            colors = std::move(_colors);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::GeoLineStrings> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(
            const archetypes::GeoLineStrings& archetype
        );
    };
} // namespace rerun
