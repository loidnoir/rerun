# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/views/time_series.fbs".

from __future__ import annotations

__all__ = ["TimeSeriesView"]


from ..._baseclasses import AsComponents
from ...datatypes import EntityPathLike, Utf8Like
from .. import archetypes as blueprint_archetypes
from .. import components as blueprint_components
from ..api import SpaceView, SpaceViewContentsLike


class TimeSeriesView(SpaceView):
    """**View**: A time series view."""

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: SpaceViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
        visible: blueprint_components.VisibleLike | None = None,
        axis_y: blueprint_archetypes.ScalarAxis | None = None,
        plot_legend: blueprint_archetypes.PlotLegend | blueprint_components.Corner2D | None = None,
        time_ranges: blueprint_archetypes.VisibleTimeRanges | None = None,
    ) -> None:
        """
        Construct a blueprint for a new TimeSeriesView view.

        Parameters
        ----------
        origin:
            The `EntityPath` to use as the origin of this view.
            All other entities will be transformed to be displayed relative to this origin.
        contents:
            The contents of the view specified as a query expression.
            This is either a single expression, or a list of multiple expressions.
            See [rerun.blueprint.archetypes.SpaceViewContents][].
        name:
            The display name of the view.
        visible:
            Whether this view is visible.

            Defaults to true if not specified.
        axis_y:
            Configures the vertical axis of the plot.
        plot_legend:
            Configures the legend of the plot.
        time_ranges:
            Configures which range on each timeline is shown by this view (unless specified differently per entity).

        """

        properties: dict[str, AsComponents] = {}
        if axis_y is not None:
            if not isinstance(axis_y, blueprint_archetypes.ScalarAxis):
                axis_y = blueprint_archetypes.ScalarAxis(axis_y)
            properties["ScalarAxis"] = axis_y

        if plot_legend is not None:
            if not isinstance(plot_legend, blueprint_archetypes.PlotLegend):
                plot_legend = blueprint_archetypes.PlotLegend(plot_legend)
            properties["PlotLegend"] = plot_legend

        if time_ranges is not None:
            if not isinstance(time_ranges, blueprint_archetypes.VisibleTimeRanges):
                time_ranges = blueprint_archetypes.VisibleTimeRanges(time_ranges)
            properties["VisibleTimeRanges"] = time_ranges

        super().__init__(
            class_identifier="TimeSeries",
            origin=origin,
            contents=contents,
            name=name,
            visible=visible,
            properties=properties,
        )
