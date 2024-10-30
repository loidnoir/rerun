// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/line_strips3d.fbs".

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

/// **Archetype**: 3D line strips with positions and optional colors, radii, labels, etc.
///
/// ## Examples
///
/// ### Many strips
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_line_strip3d_batch").spawn()?;
///
///     let strip1 = [[0., 0., 2.], [1., 0., 2.], [1., 1., 2.], [0., 1., 2.]];
///     let strip2 = [
///         [0., 0., 0.],
///         [0., 0., 1.],
///         [1., 0., 0.],
///         [1., 0., 1.],
///         [1., 1., 0.],
///         [1., 1., 1.],
///         [0., 1., 0.],
///         [0., 1., 1.],
///     ];
///     rec.log(
///         "strips",
///         &rerun::LineStrips3D::new([strip1.to_vec(), strip2.to_vec()])
///             .with_colors([0xFF0000FF, 0x00FF00FF])
///             .with_radii([0.025, 0.005])
///             .with_labels(["one strip here", "and one strip there"]),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/1200w.png">
///   <img src="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/full.png" width="640">
/// </picture>
/// </center>
///
/// ### Lines with scene & UI radius each
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_line_strip3d_ui_radius").spawn()?;
///
///     // A blue line with a scene unit radii of 0.01.
///     let points = [[0., 0., 0.], [0., 0., 1.], [1., 0., 0.], [1., 0., 1.]];
///     rec.log(
///         "scene_unit_line",
///         &rerun::LineStrips3D::new([points])
///             // By default, radii are interpreted as world-space units.
///             .with_radii([0.01])
///             .with_colors([rerun::Color::from_rgb(0, 0, 255)]),
///     )?;
///
///     // A red line with a ui point radii of 5.
///     // UI points are independent of zooming in Views, but are sensitive to the application UI scaling.
///     // For 100 % ui scaling, UI points are equal to pixels.
///     let points = [[3., 0., 0.], [3., 0., 1.], [4., 0., 0.], [4., 0., 1.]];
///     rec.log(
///         "ui_points_line",
///         &rerun::LineStrips3D::new([points])
///             // rerun::Radius::new_ui_points produces a radius that the viewer interprets as given in ui points.
///             .with_radii([rerun::Radius::new_ui_points(5.0)])
///             .with_colors([rerun::Color::from_rgb(255, 0, 0)]),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/1200w.png">
///   <img src="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct LineStrips3D {
    /// All the actual 3D line strips that make up the batch.
    pub strips: Vec<crate::components::LineStrip3D>,

    /// Optional radii for the line strips.
    pub radii: Option<Vec<crate::components::Radius>>,

    /// Optional colors for the line strips.
    pub colors: Option<Vec<crate::components::Color>>,

    /// Optional text labels for the line strips.
    ///
    /// If there's a single label present, it will be placed at the center of the entity.
    /// Otherwise, each instance will have its own label.
    pub labels: Option<Vec<crate::components::Text>>,

    /// Optional choice of whether the text labels should be shown by default.
    pub show_labels: Option<crate::components::ShowLabels>,

    /// Optional [`components::ClassId`][crate::components::ClassId]s for the lines.
    ///
    /// The [`components::ClassId`][crate::components::ClassId] provides colors and labels if not specified explicitly.
    pub class_ids: Option<Vec<crate::components::ClassId>>,
}

impl ::re_types_core::SizeBytes for LineStrips3D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.strips.heap_size_bytes()
            + self.radii.heap_size_bytes()
            + self.colors.heap_size_bytes()
            + self.labels.heap_size_bytes()
            + self.show_labels.heap_size_bytes()
            + self.class_ids.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::components::LineStrip3D>>::is_pod()
            && <Option<Vec<crate::components::Radius>>>::is_pod()
            && <Option<Vec<crate::components::Color>>>::is_pod()
            && <Option<Vec<crate::components::Text>>>::is_pod()
            && <Option<crate::components::ShowLabels>>::is_pod()
            && <Option<Vec<crate::components::ClassId>>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.LineStrip3D".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Radius".into(),
            "rerun.components.Color".into(),
            "rerun.components.LineStrips3DIndicator".into(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Text".into(),
            "rerun.components.ShowLabels".into(),
            "rerun.components.ClassId".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 7usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.LineStrip3D".into(),
            "rerun.components.Radius".into(),
            "rerun.components.Color".into(),
            "rerun.components.LineStrips3DIndicator".into(),
            "rerun.components.Text".into(),
            "rerun.components.ShowLabels".into(),
            "rerun.components.ClassId".into(),
        ]
    });

static REQUIRED_COMPONENT_DESCRIPTORS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
            component_name: "rerun.components.LineStrip3D".into(),
            archetype_field_name: Some("strips".into()),
        }]
    });

static RECOMMENDED_COMPONENT_DESCRIPTORS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                component_name: "rerun.components.Radius".into(),
                archetype_field_name: Some("radii".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                component_name: "rerun.components.Color".into(),
                archetype_field_name: Some("colors".into()),
            },
        ]
    });

static OPTIONAL_COMPONENT_DESCRIPTORS: once_cell::sync::Lazy<[ComponentDescriptor; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                component_name: "rerun.components.Text".into(),
                archetype_field_name: Some("labels".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                component_name: "rerun.components.ShowLabels".into(),
                archetype_field_name: Some("show_labels".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                component_name: "rerun.components.ClassId".into(),
                archetype_field_name: Some("class_ids".into()),
            },
        ]
    });

static ALL_COMPONENT_DESCRIPTORS: once_cell::sync::Lazy<[ComponentDescriptor; 6usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                component_name: "rerun.components.LineStrip3D".into(),
                archetype_field_name: Some("strips".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                component_name: "rerun.components.Radius".into(),
                archetype_field_name: Some("radii".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                component_name: "rerun.components.Color".into(),
                archetype_field_name: Some("colors".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                component_name: "rerun.components.Text".into(),
                archetype_field_name: Some("labels".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                component_name: "rerun.components.ShowLabels".into(),
                archetype_field_name: Some("show_labels".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                component_name: "rerun.components.ClassId".into(),
                archetype_field_name: Some("class_ids".into()),
            },
        ]
    });

impl LineStrips3D {
    /// The total number of components in the archetype: 1 required, 3 recommended, 3 optional
    pub const NUM_COMPONENTS: usize = 7usize;
}

/// Indicator component for the [`LineStrips3D`] [`::re_types_core::Archetype`]
pub type LineStrips3DIndicator = ::re_types_core::GenericIndicatorComponent<LineStrips3D>;

impl ::re_types_core::Archetype for LineStrips3D {
    type Indicator = LineStrips3DIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.LineStrips3D".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Line strips 3D"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: LineStrips3DIndicator = LineStrips3DIndicator::DEFAULT;
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
        let strips = {
            let array = arrays_by_name
                .get("rerun.components.LineStrip3D")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.LineStrips3D#strips")?;
            <crate::components::LineStrip3D>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.LineStrips3D#strips")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.LineStrips3D#strips")?
        };
        let radii = if let Some(array) = arrays_by_name.get("rerun.components.Radius") {
            Some({
                <crate::components::Radius>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.LineStrips3D#radii")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.LineStrips3D#radii")?
            })
        } else {
            None
        };
        let colors = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            Some({
                <crate::components::Color>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.LineStrips3D#colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.LineStrips3D#colors")?
            })
        } else {
            None
        };
        let labels = if let Some(array) = arrays_by_name.get("rerun.components.Text") {
            Some({
                <crate::components::Text>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.LineStrips3D#labels")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.LineStrips3D#labels")?
            })
        } else {
            None
        };
        let show_labels = if let Some(array) = arrays_by_name.get("rerun.components.ShowLabels") {
            <crate::components::ShowLabels>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.LineStrips3D#show_labels")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let class_ids = if let Some(array) = arrays_by_name.get("rerun.components.ClassId") {
            Some({
                <crate::components::ClassId>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.LineStrips3D#class_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.LineStrips3D#class_ids")?
            })
        } else {
            None
        };
        Ok(Self {
            strips,
            radii,
            colors,
            labels,
            show_labels,
            class_ids,
        })
    }
}

impl ::re_types_core::AsComponents for LineStrips3D {
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
            (Some(&self.strips as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::MaybeOwnedDescribedComponentBatch {
                    batch: batch.into(),
                    descriptor: ComponentDescriptor {
                        archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                        archetype_field_name: Some(("strips").into()),
                        component_name: ("rerun.components.LineStrip3D").into(),
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
                    archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
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
                    archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                    archetype_field_name: Some(("colors").into()),
                    component_name: ("rerun.components.Color").into(),
                },
            }),
            (self
                .labels
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::MaybeOwnedDescribedComponentBatch {
                batch: batch.into(),
                descriptor: ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                    archetype_field_name: Some(("labels").into()),
                    component_name: ("rerun.components.Text").into(),
                },
            }),
            (self
                .show_labels
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::MaybeOwnedDescribedComponentBatch {
                batch: batch.into(),
                descriptor: ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                    archetype_field_name: Some(("show_labels").into()),
                    component_name: ("rerun.components.ShowLabels").into(),
                },
            }),
            (self
                .class_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::MaybeOwnedDescribedComponentBatch {
                batch: batch.into(),
                descriptor: ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.LineStrips3D".into()),
                    archetype_field_name: Some(("class_ids").into()),
                    component_name: ("rerun.components.ClassId").into(),
                },
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for LineStrips3D {}

impl LineStrips3D {
    /// Create a new `LineStrips3D`.
    #[inline]
    pub fn new(
        strips: impl IntoIterator<Item = impl Into<crate::components::LineStrip3D>>,
    ) -> Self {
        Self {
            strips: strips.into_iter().map(Into::into).collect(),
            radii: None,
            colors: None,
            labels: None,
            show_labels: None,
            class_ids: None,
        }
    }

    /// Optional radii for the line strips.
    #[inline]
    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = Some(radii.into_iter().map(Into::into).collect());
        self
    }

    /// Optional colors for the line strips.
    #[inline]
    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }

    /// Optional text labels for the line strips.
    ///
    /// If there's a single label present, it will be placed at the center of the entity.
    /// Otherwise, each instance will have its own label.
    #[inline]
    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<crate::components::Text>>,
    ) -> Self {
        self.labels = Some(labels.into_iter().map(Into::into).collect());
        self
    }

    /// Optional choice of whether the text labels should be shown by default.
    #[inline]
    pub fn with_show_labels(
        mut self,
        show_labels: impl Into<crate::components::ShowLabels>,
    ) -> Self {
        self.show_labels = Some(show_labels.into());
        self
    }

    /// Optional [`components::ClassId`][crate::components::ClassId]s for the lines.
    ///
    /// The [`components::ClassId`][crate::components::ClassId] provides colors and labels if not specified explicitly.
    #[inline]
    pub fn with_class_ids(
        mut self,
        class_ids: impl IntoIterator<Item = impl Into<crate::components::ClassId>>,
    ) -> Self {
        self.class_ids = Some(class_ids.into_iter().map(Into::into).collect());
        self
    }
}
