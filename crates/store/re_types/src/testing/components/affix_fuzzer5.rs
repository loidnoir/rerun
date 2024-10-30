// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/testing/components/fuzzy.fbs".

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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AffixFuzzer5(pub Option<crate::testing::datatypes::AffixFuzzer1>);

impl ::re_types_core::SizeBytes for AffixFuzzer5 {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::testing::datatypes::AffixFuzzer1>>::is_pod()
    }
}

impl<T: Into<Option<crate::testing::datatypes::AffixFuzzer1>>> From<T> for AffixFuzzer5 {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<Option<crate::testing::datatypes::AffixFuzzer1>> for AffixFuzzer5 {
    #[inline]
    fn borrow(&self) -> &Option<crate::testing::datatypes::AffixFuzzer1> {
        &self.0
    }
}

impl std::ops::Deref for AffixFuzzer5 {
    type Target = Option<crate::testing::datatypes::AffixFuzzer1>;

    #[inline]
    fn deref(&self) -> &Option<crate::testing::datatypes::AffixFuzzer1> {
        &self.0
    }
}

impl std::ops::DerefMut for AffixFuzzer5 {
    #[inline]
    fn deref_mut(&mut self) -> &mut Option<crate::testing::datatypes::AffixFuzzer1> {
        &mut self.0
    }
}

::re_types_core::macros::impl_into_cow!(AffixFuzzer5);

impl ::re_types_core::Loggable for AffixFuzzer5 {
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::Struct(std::sync::Arc::new(vec![
            Field::new("single_float_optional", DataType::Float32, true),
            Field::new("single_string_required", DataType::Utf8, false),
            Field::new("single_string_optional", DataType::Utf8, true),
            Field::new(
                "many_floats_optional",
                DataType::List(std::sync::Arc::new(Field::new(
                    "item",
                    DataType::Float32,
                    false,
                ))),
                true,
            ),
            Field::new(
                "many_strings_required",
                DataType::List(std::sync::Arc::new(Field::new(
                    "item",
                    DataType::Utf8,
                    false,
                ))),
                false,
            ),
            Field::new(
                "many_strings_optional",
                DataType::List(std::sync::Arc::new(Field::new(
                    "item",
                    DataType::Utf8,
                    false,
                ))),
                true,
            ),
            Field::new("flattened_scalar", DataType::Float32, false),
            Field::new(
                "almost_flattened_scalar",
                <crate::testing::datatypes::FlattenedScalar>::arrow_datatype(),
                false,
            ),
            Field::new("from_parent", DataType::Boolean, true),
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
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| datum.into_owned().0).flatten();
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                _ = data0_bitmap;
                crate::testing::datatypes::AffixFuzzer1::to_arrow_opt(data0)?
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
        Ok(
            crate::testing::datatypes::AffixFuzzer1::from_arrow_opt(arrow_data)
                .with_context("rerun.testing.components.AffixFuzzer5#single_optional")?
                .into_iter()
                .map(Ok)
                .map(|res| res.map(|v| Some(Self(v))))
                .collect::<DeserializationResult<Vec<Option<_>>>>()
                .with_context("rerun.testing.components.AffixFuzzer5#single_optional")
                .with_context("rerun.testing.components.AffixFuzzer5")?,
        )
    }
}

impl ::re_types_core::Component for AffixFuzzer5 {
    #[inline]
    fn name() -> ComponentName {
        "rerun.testing.components.AffixFuzzer5".into()
    }
}
