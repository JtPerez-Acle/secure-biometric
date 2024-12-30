use crate::common::TestContext;
use secure_biometric::storage::TemplateVault;
use secure_biometric::templates::{Template, TemplateMetadata, TemplateType};
use std::time::Instant;
use tokio::task;
use std::sync::Arc;

const BATCH_SIZE: usize = 100;

#[tokio::test]
async fn test_concurrent_storage() {
    let ctx = TestContext::new();
    let vault = Arc::new(
        TemplateVault::new(ctx.temp_path())
            .await
            .expect("Failed to create vault")
    );

    const NUM_TEMPLATES: usize = 1000;
    let mut handles = vec![];
    let start = Instant::now();

    // Store templates in batches to reduce contention
    for batch_start in (0..NUM_TEMPLATES).step_by(BATCH_SIZE) {
        let vault = vault.clone();
        let batch_end = (batch_start + BATCH_SIZE).min(NUM_TEMPLATES);
        
        handles.push(task::spawn(async move {
            let mut batch_ids = Vec::new();
            for i in batch_start..batch_end {
                let template = Template::new(
                    vec![i as u8],
                    TemplateMetadata {
                        version: "1.0".to_string(),
                        template_type: TemplateType::Face,
                        quality_score: 0.95,
                        extra: serde_json::json!({}),
                    },
                );
                let id = vault.store(template).await.expect("Failed to store template");
                batch_ids.push(id);
            }
            batch_ids
        }));
    }

    // Wait for all operations to complete
    let mut all_ids = Vec::new();
    for handle in handles {
        let batch_ids = handle.await.expect("Task failed");
        all_ids.extend(batch_ids);
    }

    // Ensure data is properly persisted
    vault.flush().await.expect("Failed to flush data");

    let duration = start.elapsed();
    println!(
        "Stored {} templates in {:?} ({:?} per template)",
        NUM_TEMPLATES,
        duration,
        duration / NUM_TEMPLATES as u32
    );

    // Verify all templates can be retrieved
    for id in all_ids {
        vault.get(id).await.expect("Failed to retrieve template");
    }
}

#[tokio::test]
async fn test_bulk_retrieval() {
    let ctx = TestContext::new();
    let vault = Arc::new(
        TemplateVault::new(ctx.temp_path())
            .await
            .expect("Failed to create vault")
    );

    // Store test templates
    const NUM_TEMPLATES: usize = 100;
    let mut ids = Vec::new();

    for i in 0..NUM_TEMPLATES {
        let template = Template::new(
            vec![i as u8],
            TemplateMetadata {
                version: "1.0".to_string(),
                template_type: TemplateType::Face,
                quality_score: 0.95,
                extra: serde_json::json!({}),
            },
        );
        let id = vault.store(template).await.expect("Failed to store template");
        ids.push(id);
    }

    vault.flush().await.expect("Failed to flush data");

    // Measure retrieval performance
    let start = Instant::now();
    let mut handles = vec![];

    for batch in ids.chunks(BATCH_SIZE) {
        let vault = vault.clone();
        let batch_ids = batch.to_vec();
        
        handles.push(task::spawn(async move {
            for id in batch_ids {
                vault.get(id).await.expect("Failed to retrieve template");
            }
        }));
    }

    for handle in handles {
        handle.await.expect("Task failed");
    }

    let duration = start.elapsed();
    println!(
        "Retrieved {} templates in {:?} ({:?} per template)",
        NUM_TEMPLATES,
        duration,
        duration / NUM_TEMPLATES as u32
    );
}

#[tokio::test]
async fn test_memory_usage() {
    let ctx = TestContext::new();
    let vault = Arc::new(
        TemplateVault::new(ctx.temp_path())
            .await
            .expect("Failed to create vault")
    );

    // Store and retrieve templates while monitoring memory
    const NUM_TEMPLATES: usize = 1000;
    let mut ids = Vec::new();

    // Store templates
    for i in 0..NUM_TEMPLATES {
        let template = Template::new(
            vec![i as u8; 1024], // 1KB of data per template
            TemplateMetadata {
                version: "1.0".to_string(),
                template_type: TemplateType::Face,
                quality_score: 0.95,
                extra: serde_json::json!({}),
            },
        );
        let id = vault.store(template).await.expect("Failed to store template");
        ids.push(id);

        // Periodically flush
        if i % BATCH_SIZE == 0 {
            vault.flush().await.expect("Failed to flush");
        }
    }

    // Final flush
    vault.flush().await.expect("Failed to flush");
}

#[tokio::test]
async fn test_database_performance() {
    let ctx = TestContext::new();
    let vault = Arc::new(
        TemplateVault::new(ctx.temp_path())
            .await
            .expect("Failed to create vault")
    );

    // Test write performance
    const NUM_WRITES: usize = 1000;
    let start = Instant::now();

    for i in 0..NUM_WRITES {
        let template = Template::new(
            vec![i as u8],
            TemplateMetadata {
                version: "1.0".to_string(),
                template_type: TemplateType::Face,
                quality_score: 0.95,
                extra: serde_json::json!({}),
            },
        );
        vault.store(template).await.expect("Failed to store template");

        // Periodically flush
        if i % BATCH_SIZE == 0 {
            vault.flush().await.expect("Failed to flush");
        }
    }

    let duration = start.elapsed();
    println!(
        "Database write performance: {} ops/sec",
        NUM_WRITES as f64 / duration.as_secs_f64()
    );

    // Final flush
    vault.flush().await.expect("Failed to flush");
}
