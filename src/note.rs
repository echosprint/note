use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: String,
    pub content: String,
    pub timestamp: DateTime<Local>,
}

impl Note {
    pub fn new(content: String, existing_ids: &[String]) -> Self {
        let timestamp = Local::now();
        let id = Self::generate_unique_id(&content, &timestamp, existing_ids);
        Self {
            id,
            content,
            timestamp,
        }
    }
    
    fn generate_unique_id(content: &str, timestamp: &DateTime<Local>, existing_ids: &[String]) -> String {
        let mut counter = 0u32;
        loop {
            let mut hasher = DefaultHasher::new();
            content.hash(&mut hasher);
            timestamp.timestamp_nanos_opt().unwrap_or(0).hash(&mut hasher);
            // Add counter for uniqueness in case of collision
            counter.hash(&mut hasher);
            
            let hash = hasher.finish();
            let id: String = format!("{:x}", hash).chars().take(4).collect();
            
            // Check if this ID already exists
            if !existing_ids.contains(&id) {
                return id;
            }
            
            counter += 1;
            // Safety check to prevent infinite loop (though extremely unlikely)
            if counter > 65536 {
                // Fallback to a longer ID if we somehow exhaust all possibilities
                return format!("{:x}", hash).chars().take(8).collect();
            }
        }
    }
}

#[derive(Debug)]
pub enum RemoveResult {
    Removed(String),
    NotFound,
    Ambiguous(Vec<String>),
}