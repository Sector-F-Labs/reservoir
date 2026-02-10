use serde::{Deserialize, Serialize};

/// Represents a message in conversations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}
