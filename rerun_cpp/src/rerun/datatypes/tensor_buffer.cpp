// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/tensor_buffer.fbs".

#include "tensor_buffer.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun::datatypes {}

namespace rerun {
    const std::shared_ptr<arrow::DataType>& Loggable<datatypes::TensorBuffer>::arrow_datatype() {
        static const auto datatype = arrow::dense_union({
            arrow::field("_null_markers", arrow::null(), true, nullptr),
            arrow::field("U8", arrow::list(arrow::field("item", arrow::uint8(), false)), false),
            arrow::field("U16", arrow::list(arrow::field("item", arrow::uint16(), false)), false),
            arrow::field("U32", arrow::list(arrow::field("item", arrow::uint32(), false)), false),
            arrow::field("U64", arrow::list(arrow::field("item", arrow::uint64(), false)), false),
            arrow::field("I8", arrow::list(arrow::field("item", arrow::int8(), false)), false),
            arrow::field("I16", arrow::list(arrow::field("item", arrow::int16(), false)), false),
            arrow::field("I32", arrow::list(arrow::field("item", arrow::int32(), false)), false),
            arrow::field("I64", arrow::list(arrow::field("item", arrow::int64(), false)), false),
            arrow::field("F16", arrow::list(arrow::field("item", arrow::float16(), false)), false),
            arrow::field("F32", arrow::list(arrow::field("item", arrow::float32(), false)), false),
            arrow::field("F64", arrow::list(arrow::field("item", arrow::float64(), false)), false),
        });
        return datatype;
    }

    Result<std::shared_ptr<arrow::Array>> Loggable<datatypes::TensorBuffer>::to_arrow(
        const datatypes::TensorBuffer* instances, size_t num_instances
    ) {
        // TODO(andreas): Allow configuring the memory pool.
        arrow::MemoryPool* pool = arrow::default_memory_pool();
        auto datatype = arrow_datatype();

        ARROW_ASSIGN_OR_RAISE(auto builder, arrow::MakeBuilder(datatype, pool))
        if (instances && num_instances > 0) {
            RR_RETURN_NOT_OK(Loggable<datatypes::TensorBuffer>::fill_arrow_array_builder(
                static_cast<arrow::DenseUnionBuilder*>(builder.get()),
                instances,
                num_instances
            ));
        }
        std::shared_ptr<arrow::Array> array;
        ARROW_RETURN_NOT_OK(builder->Finish(&array));
        return array;
    }

    rerun::Error Loggable<datatypes::TensorBuffer>::fill_arrow_array_builder(
        arrow::DenseUnionBuilder* builder, const datatypes::TensorBuffer* elements,
        size_t num_elements
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

        ARROW_RETURN_NOT_OK(builder->Reserve(static_cast<int64_t>(num_elements)));
        for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
            const auto& union_instance = elements[elem_idx];
            ARROW_RETURN_NOT_OK(builder->Append(static_cast<int8_t>(union_instance.get_union_tag()))
            );

            auto variant_index = static_cast<int>(union_instance.get_union_tag());
            auto variant_builder_untyped = builder->child_builder(variant_index).get();

            using TagType = datatypes::detail::TensorBufferTag;
            switch (union_instance.get_union_tag()) {
                case TagType::None: {
                    ARROW_RETURN_NOT_OK(variant_builder_untyped->AppendNull());
                } break;
                case TagType::U8: {
                    auto variant_builder =
                        static_cast<arrow::ListBuilder*>(variant_builder_untyped);
                    ARROW_RETURN_NOT_OK(variant_builder->Append());
                    auto value_builder =
                        static_cast<arrow::UInt8Builder*>(variant_builder->value_builder());
                    ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                        union_instance.get_union_data().u8.data(),
                        static_cast<int64_t>(union_instance.get_union_data().u8.size())
                    ));
                } break;
                case TagType::U16: {
                    auto variant_builder =
                        static_cast<arrow::ListBuilder*>(variant_builder_untyped);
                    ARROW_RETURN_NOT_OK(variant_builder->Append());
                    auto value_builder =
                        static_cast<arrow::UInt16Builder*>(variant_builder->value_builder());
                    ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                        union_instance.get_union_data().u16.data(),
                        static_cast<int64_t>(union_instance.get_union_data().u16.size())
                    ));
                } break;
                case TagType::U32: {
                    auto variant_builder =
                        static_cast<arrow::ListBuilder*>(variant_builder_untyped);
                    ARROW_RETURN_NOT_OK(variant_builder->Append());
                    auto value_builder =
                        static_cast<arrow::UInt32Builder*>(variant_builder->value_builder());
                    ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                        union_instance.get_union_data().u32.data(),
                        static_cast<int64_t>(union_instance.get_union_data().u32.size())
                    ));
                } break;
                case TagType::U64: {
                    auto variant_builder =
                        static_cast<arrow::ListBuilder*>(variant_builder_untyped);
                    ARROW_RETURN_NOT_OK(variant_builder->Append());
                    auto value_builder =
                        static_cast<arrow::UInt64Builder*>(variant_builder->value_builder());
                    ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                        union_instance.get_union_data().u64.data(),
                        static_cast<int64_t>(union_instance.get_union_data().u64.size())
                    ));
                } break;
                case TagType::I8: {
                    auto variant_builder =
                        static_cast<arrow::ListBuilder*>(variant_builder_untyped);
                    ARROW_RETURN_NOT_OK(variant_builder->Append());
                    auto value_builder =
                        static_cast<arrow::Int8Builder*>(variant_builder->value_builder());
                    ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                        union_instance.get_union_data().i8.data(),
                        static_cast<int64_t>(union_instance.get_union_data().i8.size())
                    ));
                } break;
                case TagType::I16: {
                    auto variant_builder =
                        static_cast<arrow::ListBuilder*>(variant_builder_untyped);
                    ARROW_RETURN_NOT_OK(variant_builder->Append());
                    auto value_builder =
                        static_cast<arrow::Int16Builder*>(variant_builder->value_builder());
                    ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                        union_instance.get_union_data().i16.data(),
                        static_cast<int64_t>(union_instance.get_union_data().i16.size())
                    ));
                } break;
                case TagType::I32: {
                    auto variant_builder =
                        static_cast<arrow::ListBuilder*>(variant_builder_untyped);
                    ARROW_RETURN_NOT_OK(variant_builder->Append());
                    auto value_builder =
                        static_cast<arrow::Int32Builder*>(variant_builder->value_builder());
                    ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                        union_instance.get_union_data().i32.data(),
                        static_cast<int64_t>(union_instance.get_union_data().i32.size())
                    ));
                } break;
                case TagType::I64: {
                    auto variant_builder =
                        static_cast<arrow::ListBuilder*>(variant_builder_untyped);
                    ARROW_RETURN_NOT_OK(variant_builder->Append());
                    auto value_builder =
                        static_cast<arrow::Int64Builder*>(variant_builder->value_builder());
                    ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                        union_instance.get_union_data().i64.data(),
                        static_cast<int64_t>(union_instance.get_union_data().i64.size())
                    ));
                } break;
                case TagType::F16: {
                    auto variant_builder =
                        static_cast<arrow::ListBuilder*>(variant_builder_untyped);
                    ARROW_RETURN_NOT_OK(variant_builder->Append());
                    auto value_builder =
                        static_cast<arrow::HalfFloatBuilder*>(variant_builder->value_builder());
                    const rerun::half* values = union_instance.get_union_data().f16.data();
                    ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                        reinterpret_cast<const uint16_t*>(values),
                        static_cast<int64_t>(union_instance.get_union_data().f16.size())
                    ));
                } break;
                case TagType::F32: {
                    auto variant_builder =
                        static_cast<arrow::ListBuilder*>(variant_builder_untyped);
                    ARROW_RETURN_NOT_OK(variant_builder->Append());
                    auto value_builder =
                        static_cast<arrow::FloatBuilder*>(variant_builder->value_builder());
                    ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                        union_instance.get_union_data().f32.data(),
                        static_cast<int64_t>(union_instance.get_union_data().f32.size())
                    ));
                } break;
                case TagType::F64: {
                    auto variant_builder =
                        static_cast<arrow::ListBuilder*>(variant_builder_untyped);
                    ARROW_RETURN_NOT_OK(variant_builder->Append());
                    auto value_builder =
                        static_cast<arrow::DoubleBuilder*>(variant_builder->value_builder());
                    ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                        union_instance.get_union_data().f64.data(),
                        static_cast<int64_t>(union_instance.get_union_data().f64.size())
                    ));
                } break;
                default:
                    assert(false && "unreachable");
            }
        }

        return Error::ok();
    }
} // namespace rerun
