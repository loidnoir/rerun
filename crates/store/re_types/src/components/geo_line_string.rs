// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/components/geo_line_string.fbs".

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
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: A geospatial line string expressed in [EPSG:4326](https://epsg.io/4326) latitude and longitude (North/East-positive degrees).
#[derive(Clone, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct GeoLineString(pub Vec<crate::datatypes::DVec2D>);

impl ::re_types_core::SizeBytes for GeoLineString {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::datatypes::DVec2D>>::is_pod()
    }
}

impl<I: Into<crate::datatypes::DVec2D>, T: IntoIterator<Item = I>> From<T> for GeoLineString {
    fn from(v: T) -> Self {
        Self(v.into_iter().map(|v| v.into()).collect())
    }
}

::re_types_core::macros::impl_into_cow!(GeoLineString);

impl ::re_types_core::Loggable for GeoLineString {
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::List(std::sync::Arc::new(Field::new(
            "item",
            <crate::datatypes::DVec2D>::arrow_datatype(),
            false,
        )))
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        #![allow(clippy::manual_is_variant_and)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| datum.into_owned().0);
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                let offsets = arrow2::offset::Offsets::<i32>::try_from_lengths(
                    data0
                        .iter()
                        .map(|opt| opt.as_ref().map_or(0, |datum| datum.len())),
                )?
                .into();
                let data0_inner_data: Vec<_> = data0.into_iter().flatten().flatten().collect();
                let data0_inner_bitmap: Option<arrow2::bitmap::Bitmap> = None;
                ListArray::try_new(
                    Self::arrow_datatype(),
                    offsets,
                    {
                        use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                        let data0_inner_data_inner_data: Vec<_> = data0_inner_data
                            .into_iter()
                            .map(|datum| datum.0)
                            .flatten()
                            .collect();
                        let data0_inner_data_inner_bitmap: Option<arrow2::bitmap::Bitmap> = None;
                        FixedSizeListArray::new(
                            DataType::FixedSizeList(
                                std::sync::Arc::new(Field::new("item", DataType::Float64, false)),
                                2usize,
                            ),
                            PrimitiveArray::new(
                                DataType::Float64,
                                data0_inner_data_inner_data.into_iter().collect(),
                                data0_inner_data_inner_bitmap,
                            )
                            .boxed(),
                            data0_inner_bitmap,
                        )
                        .boxed()
                    },
                    data0_bitmap,
                )?
                .boxed()
            }
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::ListArray<i32>>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.components.GeoLineString#lat_lon")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let arrow_data_inner = {
                    let arrow_data_inner = &**arrow_data.values();
                    {
                        let arrow_data_inner = arrow_data_inner
                            .as_any()
                            .downcast_ref::<arrow2::array::FixedSizeListArray>()
                            .ok_or_else(|| {
                                let expected = DataType::FixedSizeList(
                                    std::sync::Arc::new(Field::new(
                                        "item",
                                        DataType::Float64,
                                        false,
                                    )),
                                    2usize,
                                );
                                let actual = arrow_data_inner.data_type().clone();
                                DeserializationError::datatype_mismatch(expected, actual)
                            })
                            .with_context("rerun.components.GeoLineString#lat_lon")?;
                        if arrow_data_inner.is_empty() {
                            Vec::new()
                        } else {
                            let offsets = (0..)
                                .step_by(2usize)
                                .zip((2usize..).step_by(2usize).take(arrow_data_inner.len()));
                            let arrow_data_inner_inner = {
                                let arrow_data_inner_inner = &**arrow_data_inner.values();
                                arrow_data_inner_inner
                                    .as_any()
                                    .downcast_ref::<Float64Array>()
                                    .ok_or_else(|| {
                                        let expected = DataType::Float64;
                                        let actual = arrow_data_inner_inner.data_type().clone();
                                        DeserializationError::datatype_mismatch(expected, actual)
                                    })
                                    .with_context("rerun.components.GeoLineString#lat_lon")?
                                    .into_iter()
                                    .map(|opt| opt.copied())
                                    .collect::<Vec<_>>()
                            };
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                offsets,
                                arrow_data_inner.validity(),
                            )
                            .map(|elem| {
                                elem.map(|(start, end): (usize, usize)| {
                                    debug_assert!(end - start == 2usize);
                                    if end > arrow_data_inner_inner.len() {
                                        return Err(DeserializationError::offset_slice_oob(
                                            (start, end),
                                            arrow_data_inner_inner.len(),
                                        ));
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data =
                                        unsafe { arrow_data_inner_inner.get_unchecked(start..end) };
                                    let data = data.iter().cloned().map(Option::unwrap_or_default);

                                    // NOTE: Unwrapping cannot fail: the length must be correct.
                                    #[allow(clippy::unwrap_used)]
                                    Ok(array_init::from_iter(data).unwrap())
                                })
                                .transpose()
                            })
                            .map(|res_or_opt| {
                                res_or_opt
                                    .map(|res_or_opt| res_or_opt.map(crate::datatypes::DVec2D))
                            })
                            .collect::<DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                    .collect::<Vec<_>>()
                };
                let offsets = arrow_data.offsets();
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    offsets.iter().zip(offsets.lengths()),
                    arrow_data.validity(),
                )
                .map(|elem| {
                    elem.map(|(start, len)| {
                        let start = *start as usize;
                        let end = start + len;
                        if end > arrow_data_inner.len() {
                            return Err(DeserializationError::offset_slice_oob(
                                (start, end),
                                arrow_data_inner.len(),
                            ));
                        }

                        #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                        let data = unsafe { arrow_data_inner.get_unchecked(start..end) };
                        let data = data
                            .iter()
                            .cloned()
                            .map(Option::unwrap_or_default)
                            .collect();
                        Ok(data)
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<Option<_>>>>()?
            }
            .into_iter()
        }
        .map(|v| v.ok_or_else(DeserializationError::missing_data))
        .map(|res| res.map(|v| Some(Self(v))))
        .collect::<DeserializationResult<Vec<Option<_>>>>()
        .with_context("rerun.components.GeoLineString#lat_lon")
        .with_context("rerun.components.GeoLineString")?)
    }
}

impl ::re_types_core::Component for GeoLineString {
    #[inline]
    fn name() -> ComponentName {
        "rerun.components.GeoLineString".into()
    }
}
