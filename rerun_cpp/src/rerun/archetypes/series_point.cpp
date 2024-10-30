// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/series_point.fbs".

#include "series_point.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {}

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<archetypes::SeriesPoint>::serialize(
        const archetypes::SeriesPoint& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(5);

        if (archetype.color.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.color.value(),
                "rerun.archetypes.SeriesPoint",
                "color"
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.marker.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.marker.value(),
                "rerun.archetypes.SeriesPoint",
                "marker"
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.name.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.name.value(),
                "rerun.archetypes.SeriesPoint",
                "name"
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.marker_size.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.marker_size.value(),
                "rerun.archetypes.SeriesPoint",
                "marker_size"
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto indicator = SeriesPoint::IndicatorComponent();
            auto result = ComponentBatch::from_loggable(indicator, "rerun.archetypes.SeriesPoint");
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
