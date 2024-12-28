use std::sync::{Arc, Mutex};
use thiserror::Error;
use qdrant::client::QdrantClient;
use rust_bert::pipelines::question_answering::{QuestionAnsweringModel, QaInput};
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};
use tiktoken::get_bpe_from_model;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Error, Debug)]
pub enum RagError {
    #[error("Embedding error: {0}")]
    EmbeddingError(String),
    
    #[error("Vector store error: {0}")]
    VectorStoreError(String),
    
    #[error("LLM error: {0}")]
    LlmError(String),
    
    #[error("Tokenization error: {0}")]
    TokenizationError(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct MemoryEntry {
    timestamp: DateTime<Utc>,
    question: String,
    answer: String,
    sources: Vec<String>,
}

#[derive(Clone)]
pub struct RagService {
    qdrant_client: Arc<QdrantClient>,
    embedding_model: Arc<QuestionAnsweringModel>,
    llm_client: Arc<openai::Client>,
    memory: Arc<Mutex<Vec<MemoryEntry>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RagResponse {
    pub answer: String,
    pub sources: Vec<String>,
    pub confidence: f32,
}

impl RagService {
    pub fn new(
        qdrant_url: &str,
        embedding_model: QuestionAnsweringModel,
        openai_api_key: &str,
    ) -> Self {
        Self {
            qdrant_client: Arc::new(QdrantClient::new(qdrant_url).unwrap()),
            embedding_model: Arc::new(embedding_model),
            llm_client: Arc::new(openai::Client::new(openai_api_key)),
            memory: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_to_memory(&self, question: &str, answer: &str, sources: &[String]) {
        let entry = MemoryEntry {
            timestamp: Utc::now(),
            question: question.to_string(),
            answer: answer.to_string(),
            sources: sources.to_vec(),
        };
        
        if let Ok(mut memory) = self.memory.lock() {
            memory.push(entry);
        }
    }

    pub fn get_memory(&self, max_entries: usize) -> Vec<MemoryEntry> {
        self.memory.lock()
            .map(|memory| {
                let len = memory.len();
                memory[len.saturating_sub(max_entries)..].to_vec()
            })
            .unwrap_or_default()
    }

    pub async fn query(&self, question: &str, collection_name: &str) -> Result<RagResponse, RagError> {
        // Step 1: Generate embedding for the question
        let embedding = self.generate_embedding(question)?;
        
        // Step 2: Search vector store
        let search_results = self.search_vector_store(&embedding, collection_name).await?;
        
        // Step 3: Generate LLM response
        let response = self.generate_llm_response(question, &search_results).await?;
        
        Ok(response)
    }

    fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, RagError> {
        let qa_input = QaInput {
            question: text.to_string(),
            context: "".to_string(),
        };
        
        self.embedding_model
            .predict(&[qa_input], 1, 128)
            .map(|results| results[0].start)
            .map_err(|e| RagError::EmbeddingError(e.to_string()))
    }

    async fn search_vector_store(
        &self,
        embedding: &[f32],
        collection_name: &str,
    ) -> Result<Vec<String>, RagError> {
        self.qdrant_client
            .search_points(collection_name, embedding.to_vec(), 5)
            .await
            .map(|results| {
                results
                    .into_iter()
                    .map(|point| point.payload["text"].as_str().unwrap().to_string())
                    .collect()
            })
            .map_err(|e| RagError::VectorStoreError(e.to_string()))
    }

    async fn generate_llm_response(
        &self,
        question: &str,
        context: &[String],
    ) -> Result<RagResponse, RagError> {
        // Get recent memory
        let memory = self.get_memory(3);
        
        // Build message history
        let mut messages = vec![
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::System,
                content: "You are a helpful assistant that answers questions based on the provided context and previous conversation history.".to_string(),
            },
        ];

        // Add memory entries
        for entry in memory {
            messages.push(ChatCompletionMessage {
                role: ChatCompletionMessageRole::Assistant,
                content: format!("Previous Q&A:\nQ: {}\nA: {}", entry.question, entry.answer),
            });
        }

        // Add current context
        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: format!("Question: {}\nContext: {}", question, context.join("\n")),
        });

        let response = self.llm_client
            .chat()
            .create(ChatCompletion {
                model: "gpt-4".to_string(),
                messages,
                temperature: 0.7,
                max_tokens: 512,
                ..Default::default()
            })
            .await
            .map_err(|e| RagError::LlmError(e.to_string()))?;

        let response = RagResponse {
            answer: response.choices[0].message.content.clone(),
            sources: context.to_vec(),
            confidence: 1.0, // Placeholder for confidence score
        };

        // Add to memory
        self.add_to_memory(question, &response.answer, &response.sources);

        Ok(response)
    }

    pub fn count_tokens(&self, text: &str, model_name: &str) -> Result<usize, RagError> {
        let bpe = get_bpe_from_model(model_name)
            .map_err(|e| RagError::TokenizationError(e.to_string()))?;
        Ok(bpe.encode_with_special_tokens(text).len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Server};
    use rust_bert::pipelines::question_answering::QuestionAnsweringConfig;

    async fn create_test_service(mock_server: &Server) -> RagService {
        let config = QuestionAnsweringConfig::default();
        let model = QuestionAnsweringModel::new(config).unwrap();
        RagService::new(&mock_server.url(), model, "test-api-key")
    }

    #[tokio::test]
    async fn test_query() {
        let mut mock_server = Server::new();
    
        // Mock Qdrant search endpoint
        let _m = mock("POST", "/collections/test_collection/points/search")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": [{"payload": {"text": "test context"}}]}"#)
            .create();
        
        // Mock OpenAI chat endpoint
        let _m2 = mock("POST", "/v1/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"choices": [{"message": {"content": "test answer"}}]}"#)
            .create();
        
        let service = create_test_service(&mock_server).await;
        let result = service.query("test question", "test_collection").await.unwrap();
    
        assert_eq!(result.answer, "test answer");
        assert_eq!(result.sources, vec!["test context"]);

        // Test memory
        let memory = service.get_memory(1);
        assert_eq!(memory.len(), 1);
        assert_eq!(memory[0].question, "test question");
        assert_eq!(memory[0].answer, "test answer");
        assert_eq!(memory[0].sources, vec!["test context"]);
    }

    #[tokio::test]
    async fn test_memory_limits() {
        let mock_server = Server::new();
        let service = create_test_service(&mock_server).await;

        // Add multiple entries
        for i in 0..5 {
            service.add_to_memory(
                &format!("question {}", i),
                &format!("answer {}", i),
                &[format!("source {}", i)]
            );
        }

        // Test memory limits
        let memory = service.get_memory(3);
        assert_eq!(memory.len(), 3);
        assert_eq!(memory[0].question, "question 2");
        assert_eq!(memory[2].question, "question 4");
    }

    #[test]
    fn test_count_tokens() {
        let mock_server = Server::new();
        let service = create_test_service(&mock_server).await;
        let token_count = service.count_tokens("Hello world!", "gpt-4").unwrap();
        assert!(token_count > 0);
    }

    #[tokio::test]
    async fn test_generate_embedding() {
        let mock_server = Server::new();
        let service = create_test_service(&mock_server).await;
        let embedding = service.generate_embedding("test").unwrap();
        assert!(!embedding.is_empty());
    }
}
