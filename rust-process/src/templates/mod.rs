mod template;
mod error;

pub use error::TemplateError;
pub use template::{Template, TemplateMetadata, TemplateType};

pub type Result<T> = std::result::Result<T, TemplateError>;

#[cfg(test)]
mod tests {
    use super::*;
    use template::{TemplateMetadata, TemplateType};

    #[test]
    fn test_template_creation() {
        let template = Template::new(
            vec![1, 2, 3, 4],
            TemplateMetadata {
                version: "1.0".to_string(),
                template_type: TemplateType::Face,
                quality_score: 0.95,
                extra: serde_json::json!({}),
            },
        );

        assert_eq!(template.data, vec![1, 2, 3, 4]);
        assert!(template.validate());
    }
}
