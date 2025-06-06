use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A parameter given to a query.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryParameter {
    /// Optional. If unset, this is a positional parameter. Otherwise, should be unique within a query.
    pub name: Option<String>,
    /// Required. The type of this parameter.
    pub parameter_type: Option<QueryParameterType>,
    /// Required. The value of this parameter.
    pub parameter_value: Option<QueryParameterValue>,
}

/// The type of a query parameter.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryParameterType {
    /// Optional. The type of the array's elements, if this is an array.
    pub array_type: Option<Box<QueryParameterType>>,
    /// Optional. The element type of the range, if this is a range.
    pub range_element_type: Option<Box<QueryParameterType>>,
    /// Optional. The types of the fields of this struct, in order, if this is a struct.
    pub struct_types: Option<Vec<StructTypeField>>,
    /// Required. The top level type of this field.
    pub r#type: String,
}

/// The type of a struct parameter field.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StructTypeField {
    /// Optional. Human-oriented description of the field.
    pub description: Option<String>,
    /// Optional. The name of this field.
    pub name: Option<String>,
    /// Required. The type of this field.
    pub r#type: QueryParameterType,
}

/// The value of a query parameter.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryParameterValue {
    /// Optional. The array values, if this is an array type.
    pub array_values: Option<Vec<QueryParameterValue>>,
    /// Optional. The range value, if this is a range type.
    pub range_value: Option<Box<RangeValue>>,
    /// The struct field values.
    pub struct_values: Option<HashMap<String, QueryParameterValue>>,
    /// Optional. The value of this value, if a simple scalar type.
    pub value: Option<String>,
}

/// Represents the value of a range.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RangeValue {
    /// Optional. The end value of the range. A missing value represents an unbounded end.
    pub end: Option<Box<QueryParameterValue>>,
    /// Optional. The start value of the range. A missing value represents an unbounded start.
    pub start: Option<Box<QueryParameterValue>>,
}
