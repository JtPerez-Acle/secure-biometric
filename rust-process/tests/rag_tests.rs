use rust_process::rag::{RagService, RagError};
use rust_bert::pipelines::question_answering::QuestionAnsweringConfig;
use mockito::{mock, Server};

#[tokio::test]
async fn test_rag_workflow() {
    let mut mock_server = Server::new();
    
    // Setup mocks
    let _m1 = mock("POST", "/collections/test/points/search")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"result": [{"payload": {"text": "test context"}}]}"#)
        .create();
        
    let _m2 = mock("POST", "/v1/chat/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"choices": [{"message": {"content": "test answer"}}]}"#)
        .create();
    
    // Create test service
    let config = QuestionAnsweringConfig::default();
    let model = rust_bert::pipelines::question_answering::QuestionAnsweringModel::new(config).unwrap();
    let service = RagService::new(&mock_server.url(), model, "test-api-key");
    
    // Test query
    let result = service.query("test question", "test").await.unwrap();
    
    assert_eq!(result.answer, "test answer");
    assert_eq!(result.sources, vec!["test context"]);
}

#[tokio::test]
async fn test_error_handling() {
    let mock_server = Server::new();
    
    // Create test service
    let config = QuestionAnsweringConfig::default();
    let model = rust_bert::pipelines::question_answering::QuestionAnsweringModel::new(config).unwrap();
    let service = RagService::new(&mock_server.url(), model, "test-api-key");
    
    // Test error cases
    let result = service.query("", "test").await;
    assert!(matches!(result, Err(RagError::EmbeddingError(_))));
}
