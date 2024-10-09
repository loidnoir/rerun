// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/graph_edges_undirected.fbs".

#include "graph_edges_undirected.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {}

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<archetypes::GraphEdgesUndirected>::serialize(
        const archetypes::GraphEdgesUndirected& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(6);

        {
            auto result = ComponentBatch::from_loggable(archetype.edges);
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.colors.has_value()) {
            auto result = ComponentBatch::from_loggable(archetype.colors.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.labels.has_value()) {
            auto result = ComponentBatch::from_loggable(archetype.labels.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.show_labels.has_value()) {
            auto result = ComponentBatch::from_loggable(archetype.show_labels.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.class_ids.has_value()) {
            auto result = ComponentBatch::from_loggable(archetype.class_ids.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto indicator = GraphEdgesUndirected::IndicatorComponent();
            auto result = ComponentBatch::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
