use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    /// Unique identifier
    pub id: Option<Uuid>,
    
    /// Binary template data
    pub data: Vec<u8>,
    
    /// Template metadata
    pub metadata: TemplateMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Template version
    pub version: String,
    
    /// Type of biometric template
    pub template_type: TemplateType,
    
    /// Quality score (0.0 to 1.0)
    pub quality_score: f32,
    
    /// Additional metadata as JSON
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateType {
    Face,
    Fingerprint,
    Iris,
    Voice,
    Other,
}

impl Template {
    /// Create a new template
    pub fn new(data: Vec<u8>, metadata: TemplateMetadata) -> Self {
        Self {
            id: None,
            data,
            metadata,
        }
    }
    
    /// Validate template data
    pub fn validate(&self) -> bool {
        // TODO: Implement proper validation
        !self.data.is_empty() && self.metadata.quality_score >= 0.0 && self.metadata.quality_score <= 1.0
    }
}
