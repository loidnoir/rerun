//! Methods for handling Arrow datamodel log ingest

use std::{borrow::Cow, collections::BTreeMap};

use arrow::{
    array::{make_array, ArrayData},
    pyarrow::PyArrowType,
};
use arrow2::{
    array::{Array, ListArray, PrimitiveArray},
    datatypes::{DataType, Field},
    offset::Offsets,
};
use pyo3::{
    exceptions::PyRuntimeError,
    types::{PyAnyMethods, PyDict, PyDictMethods},
    Bound, PyAny, PyResult,
};

use re_chunk::{Chunk, ChunkError, ChunkId, PendingRow, RowId, TimeColumn, TransportChunk};
use re_log_types::TimePoint;
use re_sdk::{ComponentDescriptor, ComponentName, EntityPath, Timeline};

/// Perform Python-to-Rust conversion for a `ComponentDescriptor`.
pub fn descriptor_to_rust(component_descr: &Bound<'_, PyAny>) -> PyResult<ComponentDescriptor> {
    let py = component_descr.py();

    let archetype_name = component_descr.getattr(pyo3::intern!(py, "archetype_name"))?;
    let archetype_name: Option<Cow<'_, str>> = if !archetype_name.is_none() {
        Some(archetype_name.extract()?)
    } else {
        None
    };

    let archetype_field_name =
        component_descr.getattr(pyo3::intern!(py, "archetype_field_name"))?;
    let archetype_field_name: Option<Cow<'_, str>> = if !archetype_field_name.is_none() {
        Some(archetype_field_name.extract()?)
    } else {
        None
    };

    let component_name = component_descr.getattr(pyo3::intern!(py, "component_name"))?;
    let component_name: Cow<'_, str> = component_name.extract()?;

    Ok(ComponentDescriptor {
        archetype_name: archetype_name.map(|s| s.as_ref().into()),
        archetype_field_name: archetype_field_name.map(|s| s.as_ref().into()),
        component_name: component_name.as_ref().into(),
    })
}

/// Perform conversion between a pyarrow array to arrow2 types.
///
/// `name` is the name of the Rerun component, and the name of the pyarrow `Field` (column name).
pub fn array_to_rust(
    arrow_array: &Bound<'_, PyAny>,
    component_descr: &ComponentDescriptor,
) -> PyResult<(Box<dyn Array>, Field)> {
    let py_array: PyArrowType<ArrayData> = arrow_array.extract()?;
    let arr1_array = make_array(py_array.0);

    let data = arr1_array.to_data();

    let arr2_array = arrow2::array::from_data(&data);

    // NOTE: Do not carry the extension metadata beyond the FFI barrier in order the match the
    // data sent by other SDKs.
    //
    // We've stopped using datatype extensions overall, as they generally have been creating more
    // problems than they have solved.
    //
    // With the addition of `Chunk` and `ChunkMetadata`, it is likely that we will get rid of extension types
    // entirely at some point, since it looks like we won't have any use for them anymore.
    //
    // Doing so will require a more extensive refactoring of the Python SDK though, so until we're absolutely
    // certain where we're going, this is a nice, painless and easily reversible solution.
    //
    // See <https://github.com/rerun-io/rerun/issues/6606>.
    let datatype = if let DataType::List(inner) = arr2_array.data_type() {
        let Field {
            name: child_name,
            data_type,
            is_nullable,
            metadata,
        } = &**inner;

        DataType::List(std::sync::Arc::new(
            Field::new(
                child_name.clone(),
                data_type.to_logical_type().clone(),
                *is_nullable,
            )
            .with_metadata(metadata.clone()),
        ))
    } else {
        arr2_array.data_type().to_logical_type().clone()
    };

    let metadata = TransportChunk::field_metadata_component_descriptor(component_descr);

    let field = Field::new(
        component_descr.component_name.to_string(),
        datatype.clone(),
        true,
    )
    .with_metadata(metadata);

    Ok((arr2_array, field))
}

/// Build a [`PendingRow`] given a '**kwargs'-style dictionary of component arrays.
pub fn build_row_from_components(
    components_per_descr: &Bound<'_, PyDict>,
    time_point: &TimePoint,
) -> PyResult<PendingRow> {
    // Create row-id as early as possible. It has a timestamp and is used to estimate e2e latency.
    // TODO(emilk): move to before we arrow-serialize the data
    let row_id = RowId::new();

    let mut components = BTreeMap::default();
    for (component_descr, array) in components_per_descr {
        let component_descr = descriptor_to_rust(&component_descr)?;
        // TODO: get rid of the useless field computation
        let (list_array, _field) = array_to_rust(&array, &component_descr)?;

        // TODO: warning on collision or something?
        components.insert(component_descr, list_array);
    }

    Ok(PendingRow {
        row_id,
        timepoint: time_point.clone(),
        components,
    })
}

// TODO: that is also a pile of shit
/// Build a [`Chunk`] given a '**kwargs'-style dictionary of component arrays.
pub fn build_chunk_from_components(
    entity_path: EntityPath,
    timelines: &Bound<'_, PyDict>,
    components_per_descr: &Bound<'_, PyDict>,
) -> PyResult<Chunk> {
    // Create chunk-id as early as possible. It has a timestamp and is used to estimate e2e latency.
    let chunk_id = ChunkId::new();

    // Extract the timeline data
    let (arrays, fields): (Vec<Box<dyn Array>>, Vec<Field>) = itertools::process_results(
        timelines.iter().map(|(component_descr, array)| {
            array_to_rust(&array, &descriptor_to_rust(&component_descr)?)
        }),
        |iter| iter.unzip(),
    )?;

    let timelines: Result<Vec<_>, ChunkError> = arrays
        .into_iter()
        .zip(fields)
        .map(|(value, field)| {
            let timeline = match field.data_type() {
                arrow2::datatypes::DataType::Int64 => {
                    Ok(Timeline::new_sequence(field.name.clone()))
                }
                arrow2::datatypes::DataType::Timestamp(_, _) => {
                    Ok(Timeline::new_temporal(field.name.clone()))
                }
                _ => Err(ChunkError::Malformed {
                    reason: format!("Invalid data_type for timeline: {}", field.name),
                }),
            }?;
            let timeline_data = value
                .as_any()
                .downcast_ref::<PrimitiveArray<i64>>()
                .ok_or_else(|| ChunkError::Malformed {
                    reason: format!("Invalid primitive array for timeline: {}", field.name),
                })?
                .clone();
            Ok((timeline, timeline_data))
        })
        .collect();

    let timelines: BTreeMap<Timeline, TimeColumn> = timelines
        .map_err(|err| PyRuntimeError::new_err(format!("Error converting temporal data: {err}")))?
        .into_iter()
        .map(|(timeline, value)| (timeline, TimeColumn::new(None, timeline, value)))
        .collect();

    // Extract the component data
    let (arrays, fields): (Vec<Box<dyn Array>>, Vec<Field>) = itertools::process_results(
        components_per_descr.iter().map(|(component_descr, array)| {
            array_to_rust(&array, &descriptor_to_rust(&component_descr)?)
        }),
        |iter| iter.unzip(),
    )?;

    let components: Result<Vec<(ComponentName, _)>, ChunkError> = arrays
        .into_iter()
        .zip(fields)
        .map(|(value, field)| {
            let batch = if let Some(batch) = value.as_any().downcast_ref::<ListArray<i32>>() {
                batch.clone()
            } else {
                let offsets = Offsets::try_from_lengths(std::iter::repeat(1).take(value.len()))
                    .map_err(|err| ChunkError::Malformed {
                        reason: format!("Failed to create offsets: {err}"),
                    })?;
                let data_type = ListArray::<i32>::default_datatype(value.data_type().clone());
                ListArray::<i32>::try_new(data_type, offsets.into(), value, None).map_err(
                    |err| ChunkError::Malformed {
                        reason: format!("Failed to wrap in List array: {err}"),
                    },
                )?
            };

            Ok((field.name.into(), batch))
        })
        .collect();

    let components = components
        .map_err(|err| PyRuntimeError::new_err(format!("Error converting component data: {err}")))?
        .into_iter()
        .collect();

    let chunk = Chunk::from_auto_row_ids(chunk_id, entity_path, timelines, components)
        .map_err(|err| PyRuntimeError::new_err(err.to_string()))?;

    Ok(chunk)
}
