# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/components/map_provider.fbs".

# You can extend this class by creating a "MapProviderExt" class in "map_provider_ext.py".

from __future__ import annotations

from typing import Literal, Sequence, Union

import pyarrow as pa

from ..._baseclasses import (
    BaseBatch,
    BaseExtensionType,
    ComponentBatchMixin,
)

__all__ = ["MapProvider", "MapProviderArrayLike", "MapProviderBatch", "MapProviderLike", "MapProviderType"]


from enum import Enum


class MapProvider(Enum):
    """**Component**: Name of the map provider to be used in Map views."""

    OpenStreetMap = 1
    """`OpenStreetMap` is the default map provider."""

    MapboxStreets = 2
    """Mapbox Streets is a minimalistic map designed by Mapbox."""

    MapboxDark = 3
    """Mapbox Dark is a dark-themed map designed by Mapbox."""

    MapboxSatellite = 4
    """Mapbox Satellite is a satellite map designed by Mapbox."""

    @classmethod
    def auto(cls, val: str | int | MapProvider) -> MapProvider:
        """Best-effort converter, including a case-insensitive string matcher."""
        if isinstance(val, MapProvider):
            return val
        if isinstance(val, int):
            return cls(val)
        try:
            return cls[val]
        except KeyError:
            val_lower = val.lower()
            for variant in cls:
                if variant.name.lower() == val_lower:
                    return variant
        raise ValueError(f"Cannot convert {val} to {cls.__name__}")

    def __str__(self) -> str:
        """Returns the variant name."""
        return self.name


MapProviderLike = Union[
    MapProvider,
    Literal[
        "MapboxDark",
        "MapboxSatellite",
        "MapboxStreets",
        "OpenStreetMap",
        "mapboxdark",
        "mapboxsatellite",
        "mapboxstreets",
        "openstreetmap",
    ],
    int,
]
MapProviderArrayLike = Union[MapProviderLike, Sequence[MapProviderLike]]


class MapProviderType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.blueprint.components.MapProvider"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.uint8(), self._TYPE_NAME)


class MapProviderBatch(BaseBatch[MapProviderArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = MapProviderType()

    @staticmethod
    def _native_to_pa_array(data: MapProviderArrayLike, data_type: pa.DataType) -> pa.Array:
        if isinstance(data, (MapProvider, int, str)):
            data = [data]

        pa_data = [MapProvider.auto(v).value if v is not None else None for v in data]  # type: ignore[redundant-expr]

        return pa.array(pa_data, type=data_type)
