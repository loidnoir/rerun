# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/archetypes/geo_line_strings.fbs".

# You can extend this class by creating a "GeoLineStringsExt" class in "geo_line_strings_ext.py".

from __future__ import annotations

from attrs import define, field

from .. import components
from .._baseclasses import (
    Archetype,
)
from .geo_line_strings_ext import GeoLineStringsExt

__all__ = ["GeoLineStrings"]


@define(str=False, repr=False, init=False)
class GeoLineStrings(GeoLineStringsExt, Archetype):
    """
    **Archetype**: Geospatial line strings with positions expressed in [EPSG:4326](https://epsg.io/4326) altitude and longitude (North/East-positive degrees), and optional colors and radii.

    Also known as "line strips" or "polylines".

    **Note**: Geospatial entities are experimental.

    Example
    -------
    ### Log a geospatial line string:
    ```python
    import rerun as rr

    rr.init("rerun_example_geo_line_strings", spawn=True)

    rr.log(
        "colorado",
        rr.GeoLineStrings(
            lat_lon=[
                [41.0000, -109.0452],
                [41.0000, -102.0415],
                [36.9931, -102.0415],
                [36.9931, -109.0452],
                [41.0000, -109.0452],
            ],
            radii=rr.Radius.ui_points(2.0),
            colors=[0, 0, 255],
        ),
    )
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/geo_line_strings_simple/5669983eb10906ace303755b5b5039cad75b917f/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/geo_line_strings_simple/5669983eb10906ace303755b5b5039cad75b917f/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/geo_line_strings_simple/5669983eb10906ace303755b5b5039cad75b917f/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/geo_line_strings_simple/5669983eb10906ace303755b5b5039cad75b917f/1200w.png">
      <img src="https://static.rerun.io/geo_line_strings_simple/5669983eb10906ace303755b5b5039cad75b917f/full.png" width="640">
    </picture>
    </center>

    """

    # __init__ can be found in geo_line_strings_ext.py

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            line_strings=None,  # type: ignore[arg-type]
            radii=None,  # type: ignore[arg-type]
            colors=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> GeoLineStrings:
        """Produce an empty GeoLineStrings, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    line_strings: components.GeoLineStringBatch = field(
        metadata={"component": "required"},
        converter=components.GeoLineStringBatch._required,  # type: ignore[misc]
    )
    # The line strings, expressed in [EPSG:4326](https://epsg.io/4326) coordinates (North/East-positive degrees).
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    radii: components.RadiusBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.RadiusBatch._optional,  # type: ignore[misc]
    )
    # Optional radii for the line strings.
    #
    # *Note*: scene units radiii are interpreted as meters. Currently, the display scale only considers the latitude of
    # the first vertex of each line string (see [this issue](https://github.com/rerun-io/rerun/issues/8013)).
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    colors: components.ColorBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.ColorBatch._optional,  # type: ignore[misc]
    )
    # Optional colors for the line strings.
    #
    # The colors are interpreted as RGB or RGBA in sRGB gamma-space,
    # As either 0-1 floats or 0-255 integers, with separate alpha.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
