use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationStage {
    Initializing,
    Uploading,
    Downloading,
    Verifying,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressMessage {
    pub operation_id: String,
    pub progress: f64,
    pub bytes_processed: usize,
    pub total_bytes: Option<usize>,
    pub stage: OperationStage,
    pub message: Option<String>,
}

impl ProgressMessage {
    pub fn new(operation_id: String) -> Self {
        Self {
            operation_id,
            progress: 0.0,
            bytes_processed: 0,
            total_bytes: None,
            stage: OperationStage::Initializing,
            message: None,
        }
    }

    pub fn with_bytes(mut self, bytes_processed: usize, total_bytes: Option<usize>) -> Self {
        self.bytes_processed = bytes_processed;
        self.total_bytes = total_bytes;
        if let Some(total) = total_bytes {
            self.progress = bytes_processed as f64 / total as f64;
        }
        self
    }

    pub fn with_stage(mut self, stage: OperationStage) -> Self {
        self.stage = stage;
        self
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }
}
