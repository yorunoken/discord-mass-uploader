use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Files {
    pub file_name: String,
    pub thread_id: String,
}
