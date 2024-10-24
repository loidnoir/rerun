// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/graph_nodes.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../component_batch.hpp"
#include "../components/color.hpp"
#include "../components/graph_node.hpp"
#include "../components/position2d.hpp"
#include "../components/show_labels.hpp"
#include "../components/text.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: A list of nodes in a graph with optional labels, colors, etc.
    struct GraphNodes {
        /// A list of node IDs.
        Collection<rerun::components::GraphNode> node_ids;

        /// Optional center positions of the nodes.
        std::optional<Collection<rerun::components::Position2D>> positions;

        /// Optional colors for the boxes.
        std::optional<Collection<rerun::components::Color>> colors;

        /// Optional text labels for the node.
        std::optional<Collection<rerun::components::Text>> labels;

        /// Optional choice of whether the text labels should be shown by default.
        std::optional<rerun::components::ShowLabels> show_labels;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.GraphNodesIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        GraphNodes() = default;
        GraphNodes(GraphNodes&& other) = default;

        explicit GraphNodes(Collection<rerun::components::GraphNode> _node_ids)
            : node_ids(std::move(_node_ids)) {}

        /// Optional center positions of the nodes.
        GraphNodes with_positions(Collection<rerun::components::Position2D> _positions) && {
            positions = std::move(_positions);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional colors for the boxes.
        GraphNodes with_colors(Collection<rerun::components::Color> _colors) && {
            colors = std::move(_colors);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional text labels for the node.
        GraphNodes with_labels(Collection<rerun::components::Text> _labels) && {
            labels = std::move(_labels);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional choice of whether the text labels should be shown by default.
        GraphNodes with_show_labels(rerun::components::ShowLabels _show_labels) && {
            show_labels = std::move(_show_labels);
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
    struct AsComponents<archetypes::GraphNodes> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(const archetypes::GraphNodes& archetype
        );
    };
} // namespace rerun
