// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/class_description_map_elem.fbs".

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

/// **Datatype**: A helper type for mapping [`datatypes::ClassId`][crate::datatypes::ClassId]s to class descriptions.
///
/// This is internal to [`components::AnnotationContext`][crate::components::AnnotationContext].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClassDescriptionMapElem {
    /// The key: the [`components::ClassId`][crate::components::ClassId].
    pub class_id: crate::datatypes::ClassId,

    /// The value: class name, color, etc.
    pub class_description: crate::datatypes::ClassDescription,
}

impl ::re_types_core::SizeBytes for ClassDescriptionMapElem {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.class_id.heap_size_bytes() + self.class_description.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::ClassId>::is_pod() && <crate::datatypes::ClassDescription>::is_pod()
    }
}

::re_types_core::macros::impl_into_cow!(ClassDescriptionMapElem);

impl ::re_types_core::Loggable for ClassDescriptionMapElem {
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::Struct(std::sync::Arc::new(vec![
            Field::new(
                "class_id",
                <crate::datatypes::ClassId>::arrow_datatype(),
                false,
            ),
            Field::new(
                "class_description",
                <crate::datatypes::ClassDescription>::arrow_datatype(),
                false,
            ),
        ]))
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
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                Self::arrow_datatype(),
                vec![
                    {
                        let (somes, class_id): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.class_id.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let class_id_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::UInt16,
                            class_id
                                .into_iter()
                                .map(|datum| datum.map(|datum| datum.0).unwrap_or_default())
                                .collect(),
                            class_id_bitmap,
                        )
                        .boxed()
                    },
                    {
                        let (somes, class_description): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum =
                                    datum.as_ref().map(|datum| datum.class_description.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let class_description_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = class_description_bitmap;
                            crate::datatypes::ClassDescription::to_arrow_opt(class_description)?
                        }
                    },
                ],
                bitmap,
            )
            .boxed()
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
                .downcast_ref::<arrow2::array::StructArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.datatypes.ClassDescriptionMapElem")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let class_id = {
                    if !arrays_by_name.contains_key("class_id") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "class_id",
                        ))
                        .with_context("rerun.datatypes.ClassDescriptionMapElem");
                    }
                    let arrow_data = &**arrays_by_name["class_id"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<UInt16Array>()
                        .ok_or_else(|| {
                            let expected = DataType::UInt16;
                            let actual = arrow_data.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.datatypes.ClassDescriptionMapElem#class_id")?
                        .into_iter()
                        .map(|opt| opt.copied())
                        .map(|res_or_opt| res_or_opt.map(crate::datatypes::ClassId))
                };
                let class_description = {
                    if !arrays_by_name.contains_key("class_description") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "class_description",
                        ))
                        .with_context("rerun.datatypes.ClassDescriptionMapElem");
                    }
                    let arrow_data = &**arrays_by_name["class_description"];
                    crate::datatypes::ClassDescription::from_arrow_opt(arrow_data)
                        .with_context("rerun.datatypes.ClassDescriptionMapElem#class_description")?
                        .into_iter()
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(class_id, class_description),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(class_id, class_description)| {
                        Ok(Self {
                            class_id: class_id
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.datatypes.ClassDescriptionMapElem#class_id")?,
                            class_description: class_description
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context(
                                    "rerun.datatypes.ClassDescriptionMapElem#class_description",
                                )?,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.ClassDescriptionMapElem")?
            }
        })
    }
}
