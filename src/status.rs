use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScannerStatus {
    pub version: String,
    pub state: ScannerState,
    #[serde(default, skip_serializing_if = "Jobs::is_empty")]
    pub jobs: Jobs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ScannerState {
    /// Idle
    Idle,
    /// Busy with some job or activity
    Processing,
    /// Calibrating, preparing the unit
    Testing,
    /// Error condition occurred
    Stopped,
    /// Unit is unavailable
    Down,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Jobs {
    pub job_info: Vec<JobInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobInfo {
    pub job_uri: String,
    pub job_uuid: String,
    pub age: u32,
    pub images_completed: u32,
    pub images_to_transfer: u32,
    pub job_state: JobState,
    pub job_state_reasons: JobStateReasons,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum JobState {
    /// End state - indicates that the job was canceled either by the remote client application
    /// (thru the eSCL interface) or by the user interacting with the scanner directly. Check
    /// [JobStateReasons] for more details.
    Canceled,
    /// End state - either an internal device error, or a communication error or a security error
    Aborted,
    /// Job is finished successfully
    Completed,
    /// The job was initiated, and the scanner is preparing the scan engine
    Pending,
    /// The scanner is processing the job and is transmitting the scan data
    Processing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobStateReasons {
    pub job_state_reason: String,
}

impl Jobs {
    fn is_empty(&self) -> bool {
        self.job_info.is_empty()
    }
}
