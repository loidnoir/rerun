// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/datatypes/class_description.fbs".

#include "class_description.hpp"

#include "annotation_info.hpp"
#include "keypoint_pair.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun::datatypes {
    const std::shared_ptr<arrow::DataType>& ClassDescription::arrow_datatype() {
        static const auto datatype = arrow::struct_({
            arrow::field("info", rerun::datatypes::AnnotationInfo::arrow_datatype(), false),
            arrow::field(
                "keypoint_annotations",
                arrow::list(
                    arrow::field("item", rerun::datatypes::AnnotationInfo::arrow_datatype(), false)
                ),
                false
            ),
            arrow::field(
                "keypoint_connections",
                arrow::list(
                    arrow::field("item", rerun::datatypes::KeypointPair::arrow_datatype(), false)
                ),
                false
            ),
        });
        return datatype;
    }

    Result<std::shared_ptr<arrow::StructBuilder>> ClassDescription::new_arrow_array_builder(
        arrow::MemoryPool* memory_pool
    ) {
        if (memory_pool == nullptr) {
            return rerun::Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
        }

        return Result(std::make_shared<arrow::StructBuilder>(
            arrow_datatype(),
            memory_pool,
            std::vector<std::shared_ptr<arrow::ArrayBuilder>>({
                rerun::datatypes::AnnotationInfo::new_arrow_array_builder(memory_pool).value,
                std::make_shared<arrow::ListBuilder>(
                    memory_pool,
                    rerun::datatypes::AnnotationInfo::new_arrow_array_builder(memory_pool).value
                ),
                std::make_shared<arrow::ListBuilder>(
                    memory_pool,
                    rerun::datatypes::KeypointPair::new_arrow_array_builder(memory_pool).value
                ),
            })
        ));
    }

    rerun::Error ClassDescription::fill_arrow_array_builder(
        arrow::StructBuilder* builder, const ClassDescription* elements, size_t num_elements
    ) {
        if (builder == nullptr) {
            return rerun::Error(ErrorCode::UnexpectedNullArgument, "Passed array builder is null.");
        }
        if (elements == nullptr) {
            return rerun::Error(
                ErrorCode::UnexpectedNullArgument,
                "Cannot serialize null pointer to arrow array."
            );
        }

        {
            auto field_builder = static_cast<arrow::StructBuilder*>(builder->field_builder(0));
            ARROW_RETURN_NOT_OK(field_builder->Reserve(static_cast<int64_t>(num_elements)));
            for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
                RR_RETURN_NOT_OK(rerun::datatypes::AnnotationInfo::fill_arrow_array_builder(
                    field_builder,
                    &elements[elem_idx].info,
                    1
                ));
            }
        }
        {
            auto field_builder = static_cast<arrow::ListBuilder*>(builder->field_builder(1));
            auto value_builder = static_cast<arrow::StructBuilder*>(field_builder->value_builder());
            ARROW_RETURN_NOT_OK(field_builder->Reserve(static_cast<int64_t>(num_elements)));
            ARROW_RETURN_NOT_OK(value_builder->Reserve(static_cast<int64_t>(num_elements * 2)));

            for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
                const auto& element = elements[elem_idx];
                ARROW_RETURN_NOT_OK(field_builder->Append());
                if (element.keypoint_annotations.size() > 0) {
                    RR_RETURN_NOT_OK(rerun::datatypes::AnnotationInfo::fill_arrow_array_builder(
                        value_builder,
                        element.keypoint_annotations.data(),
                        element.keypoint_annotations.size()
                    ));
                }
            }
        }
        {
            auto field_builder = static_cast<arrow::ListBuilder*>(builder->field_builder(2));
            auto value_builder = static_cast<arrow::StructBuilder*>(field_builder->value_builder());
            ARROW_RETURN_NOT_OK(field_builder->Reserve(static_cast<int64_t>(num_elements)));
            ARROW_RETURN_NOT_OK(value_builder->Reserve(static_cast<int64_t>(num_elements * 2)));

            for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
                const auto& element = elements[elem_idx];
                ARROW_RETURN_NOT_OK(field_builder->Append());
                if (element.keypoint_connections.size() > 0) {
                    RR_RETURN_NOT_OK(rerun::datatypes::KeypointPair::fill_arrow_array_builder(
                        value_builder,
                        element.keypoint_connections.data(),
                        element.keypoint_connections.size()
                    ));
                }
            }
        }
        ARROW_RETURN_NOT_OK(builder->AppendValues(static_cast<int64_t>(num_elements), nullptr));

        return Error::ok();
    }
} // namespace rerun::datatypes
