use serde::{Deserialize, Serialize};

/// Statistics for a query job.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatistics2 {
    /// Output only. Job resource usage breakdown by reservation. This field reported misleading information and will no longer be populated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_usage: Option<Vec<ReservationUsage>>,
}

/// Job resource usage breakdown by reservation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReservationUsage {
    /// Reservation name or "unreserved" for on-demand resource usage and multi-statement queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Total slot milliseconds used by the reservation for a particular job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_ms: Option<String>,
}
