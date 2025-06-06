use serde::{Deserialize, Serialize};

/// [Preview] Information related to sessions.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo {
    /// Output only. The id of the session.
    pub session_id: String,
}
