use serde::{Deserialize, Serialize};

/// Schema of a table.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableSchema {
    /// Describes the fields in a table.
    pub fields: Vec<TableFieldSchema>,

    /// Optional. Specifies metadata of the foreign data type definition in field schema (TableFieldSchema.foreign_type_definition).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_type_info: Option<ForeignTypeInfo>,
}

/// A field in TableSchema.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableFieldSchema {
    /// Required. The field name.
    pub name: String,

    /// Optional. The field description. The maximum length is 1,024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Required. The field data type.
    pub field_type: String,

    /// Optional. The field mode. Possible values include NULLABLE, REQUIRED and REPEATED. The default value is NULLABLE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Optional. Describes the nested schema fields if the type property is set to RECORD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<TableFieldSchema>>,

    /// Optional. Maximum length of values of this field for STRINGS or BYTES.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<String>,

    /// Optional. Precision (maximum number of total digits in base 10) for NUMERIC or BIGNUMERIC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,

    /// Optional. Scale (maximum number of digits in the fractional part in base 10) for NUMERIC or BIGNUMERIC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<String>,

    /// Optional. Field collation can be set only when the type of field is STRING.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,

    /// Optional. The policy tags attached to this field, used for field-level access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_tags: Option<PolicyTags>,

    /// Optional. Definition of the foreign data type. Only valid for top-level schema fields (not nested fields). If the type is FOREIGN, this field is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_type_definition: Option<String>,

    /// Optional. A SQL expression to specify the default value for this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value_expression: Option<String>,
    // ... other optional fields omitted for brevity ...
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PolicyTags {
    /// A list of policy tag resource names.
    pub names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForeignTypeInfo {
    /// Required. Specifies the system which defines the foreign data type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_system: Option<String>,
}
