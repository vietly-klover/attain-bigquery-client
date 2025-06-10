use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A parameter given to a query.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryParameter {
    /// Optional. If unset, this is a positional parameter. Otherwise, should be unique within a query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Required. The type of this parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<QueryParameterType>,

    /// Required. The value of this parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<QueryParameterValue>,
}

/// The type of a query parameter.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryParameterType {
    /// Optional. The type of the array's elements, if this is an array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_type: Option<Box<QueryParameterType>>,

    /// Optional. The element type of the range, if this is a range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_element_type: Option<Box<QueryParameterType>>,

    /// Optional. The types of the fields of this struct, in order, if this is a struct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub struct_types: Option<Vec<StructTypeField>>,

    /// Required. The top level type of this field.
    pub r#type: String,
}

/// The type of a struct parameter field.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StructTypeField {
    /// Optional. Human-oriented description of the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional. The name of this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Required. The type of this field.
    pub r#type: QueryParameterType,
}

/// The value of a query parameter.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryParameterValue {
    /// Optional. The array values, if this is an array type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_values: Option<Vec<QueryParameterValue>>,

    /// Optional. The range value, if this is a range type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_value: Option<Box<RangeValue>>,

    /// The struct field values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub struct_values: Option<HashMap<String, QueryParameterValue>>,

    /// Optional. The value of this value, if a simple scalar type.
    #[serde(skip_serializing_if = "Option::is_none")]
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
