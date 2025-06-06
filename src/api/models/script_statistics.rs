use serde::{Deserialize, Serialize};

/// Job statistics specific to the child job of a script.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScriptStatistics {
    /// Whether this child job was a statement or expression.
    pub evaluation_kind: String,

    /// Stack trace showing the line/column/procedure name of each frame on the stack at the point where the current evaluation happened.
    pub stack_frames: Vec<ScriptStackFrame>,
}

/// Represents the location of the statement/expression being evaluated.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScriptStackFrame {
    /// One-based end column.
    pub end_column: i32,

    /// One-based end line.
    pub end_line: i32,

    /// Name of the active procedure, empty if in a top-level script.
    pub procedure_id: String,

    /// One-based start column.
    pub start_column: i32,

    /// One-based start line.
    pub start_line: i32,

    /// Text of the current statement/expression.
    pub text: String,
}
