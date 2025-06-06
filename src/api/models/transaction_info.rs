use serde::{Deserialize, Serialize};

/// [Alpha] Information of a multi-statement transaction.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInfo {
    /// Output only. [Alpha] Id of the transaction.
    pub transaction_id: String,
}
