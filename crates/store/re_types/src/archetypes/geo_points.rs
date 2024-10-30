// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/geo_points.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::SerializationResult;
use ::re_types_core::{
    ComponentBatch, MaybeOwnedComponentBatch, MaybeOwnedDescribedComponentBatch,
};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Geospatial points with positions expressed in [EPSG:4326](https://epsg.io/4326) latitude and longitude (North/East-positive degrees), and optional colors and radii.
///
/// **Note**: Geospatial entities are experimental.
///
/// ## Example
///
/// ### Log a geospatial point
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_geo_points").spawn()?;
///
///     rec.log(
///         "rerun_hq",
///         &rerun::GeoPoints::from_lat_lon([(59.319221, 18.075631)])
///             .with_radii([rerun::Radius::new_ui_points(10.0)])
///             .with_colors([rerun::Color::from_rgb(255, 0, 0)]),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/geopoint_simple/146b0783c5aea1c1b6a9aab938879942b7c820e2/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/geopoint_simple/146b0783c5aea1c1b6a9aab938879942b7c820e2/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/geopoint_simple/146b0783c5aea1c1b6a9aab938879942b7c820e2/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/geopoint_simple/146b0783c5aea1c1b6a9aab938879942b7c820e2/1200w.png">
///   <img src="https://static.rerun.io/geopoint_simple/146b0783c5aea1c1b6a9aab938879942b7c820e2/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct GeoPoints {
    /// The [EPSG:4326](https://epsg.io/4326) coordinates for the points (North/East-positive degrees).
    pub positions: Vec<crate::components::LatLon>,

    /// Optional radii for the points, effectively turning them into circles.
    ///
    /// *Note*: scene units radiii are interpreted as meters.
    pub radii: Option<Vec<crate::components::Radius>>,

    /// Optional colors for the points.
    pub colors: Option<Vec<crate::components::Color>>,
}

impl ::re_types_core::SizeBytes for GeoPoints {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.positions.heap_size_bytes()
            + self.radii.heap_size_bytes()
            + self.colors.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::components::LatLon>>::is_pod()
            && <Option<Vec<crate::components::Radius>>>::is_pod()
            && <Option<Vec<crate::components::Color>>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.LatLon".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Radius".into(),
            "rerun.components.Color".into(),
            "rerun.components.GeoPointsIndicator".into(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 4usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.LatLon".into(),
            "rerun.components.Radius".into(),
            "rerun.components.Color".into(),
            "rerun.components.GeoPointsIndicator".into(),
        ]
    });

static REQUIRED_COMPONENT_DESCRIPTORS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GeoPoints".into()),
            component_name: "rerun.components.LatLon".into(),
            archetype_field_name: Some("positions".into()),
        }]
    });

static RECOMMENDED_COMPONENT_DESCRIPTORS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GeoPoints".into()),
                component_name: "rerun.components.Radius".into(),
                archetype_field_name: Some("radii".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GeoPoints".into()),
                component_name: "rerun.components.Color".into(),
                archetype_field_name: Some("colors".into()),
            },
        ]
    });

static OPTIONAL_COMPONENT_DESCRIPTORS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static ALL_COMPONENT_DESCRIPTORS: once_cell::sync::Lazy<[ComponentDescriptor; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GeoPoints".into()),
                component_name: "rerun.components.LatLon".into(),
                archetype_field_name: Some("positions".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GeoPoints".into()),
                component_name: "rerun.components.Radius".into(),
                archetype_field_name: Some("radii".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GeoPoints".into()),
                component_name: "rerun.components.Color".into(),
                archetype_field_name: Some("colors".into()),
            },
        ]
    });

impl GeoPoints {
    /// The total number of components in the archetype: 1 required, 3 recommended, 0 optional
    pub const NUM_COMPONENTS: usize = 4usize;
}

/// Indicator component for the [`GeoPoints`] [`::re_types_core::Archetype`]
pub type GeoPointsIndicator = ::re_types_core::GenericIndicatorComponent<GeoPoints>;

impl ::re_types_core::Archetype for GeoPoints {
    type Indicator = GeoPointsIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.GeoPoints".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Geo points"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: GeoPointsIndicator = GeoPointsIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn required_component_descriptors() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENT_DESCRIPTORS.as_slice().into()
    }

    #[inline]
    fn recommended_component_descriptors() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENT_DESCRIPTORS.as_slice().into()
    }

    #[inline]
    fn optional_component_descriptors() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENT_DESCRIPTORS.as_slice().into()
    }

    #[inline]
    fn all_component_descriptors() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENT_DESCRIPTORS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let positions = {
            let array = arrays_by_name
                .get("rerun.components.LatLon")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.GeoPoints#positions")?;
            <crate::components::LatLon>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.GeoPoints#positions")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.GeoPoints#positions")?
        };
        let radii = if let Some(array) = arrays_by_name.get("rerun.components.Radius") {
            Some({
                <crate::components::Radius>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.GeoPoints#radii")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.GeoPoints#radii")?
            })
        } else {
            None
        };
        let colors = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            Some({
                <crate::components::Color>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.GeoPoints#colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.GeoPoints#colors")?
            })
        } else {
            None
        };
        Ok(Self {
            positions,
            radii,
            colors,
        })
    }
}

impl ::re_types_core::AsComponents for GeoPoints {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        self.as_described_component_batches()
            .into_iter()
            .map(|described| described.batch)
            .collect()
    }

    fn as_described_component_batches(&self) -> Vec<MaybeOwnedDescribedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some({
                use ::re_types_core::LoggableBatch as _;
                let indicator_batch = Self::indicator();
                let indicator_name = indicator_batch.name();
                MaybeOwnedDescribedComponentBatch {
                    batch: indicator_batch,
                    descriptor: ComponentDescriptor {
                        archetype_name: Some(Self::name()),
                        component_name: indicator_name,
                        archetype_field_name: None,
                    },
                }
            }),
            (Some(&self.positions as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::MaybeOwnedDescribedComponentBatch {
                    batch: batch.into(),
                    descriptor: ComponentDescriptor {
                        archetype_name: Some("rerun.archetypes.GeoPoints".into()),
                        archetype_field_name: Some(("positions").into()),
                        component_name: ("rerun.components.LatLon").into(),
                    },
                }
            }),
            (self
                .radii
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::MaybeOwnedDescribedComponentBatch {
                batch: batch.into(),
                descriptor: ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.GeoPoints".into()),
                    archetype_field_name: Some(("radii").into()),
                    component_name: ("rerun.components.Radius").into(),
                },
            }),
            (self
                .colors
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::MaybeOwnedDescribedComponentBatch {
                batch: batch.into(),
                descriptor: ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.GeoPoints".into()),
                    archetype_field_name: Some(("colors").into()),
                    component_name: ("rerun.components.Color").into(),
                },
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for GeoPoints {}

impl GeoPoints {
    /// Create a new `GeoPoints`.
    #[inline]
    pub(crate) fn new(
        positions: impl IntoIterator<Item = impl Into<crate::components::LatLon>>,
    ) -> Self {
        Self {
            positions: positions.into_iter().map(Into::into).collect(),
            radii: None,
            colors: None,
        }
    }

    /// Optional radii for the points, effectively turning them into circles.
    ///
    /// *Note*: scene units radiii are interpreted as meters.
    #[inline]
    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = Some(radii.into_iter().map(Into::into).collect());
        self
    }

    /// Optional colors for the points.
    #[inline]
    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }
}
