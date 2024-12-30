use criterion::{criterion_group, criterion_main, Criterion};
use secure_biometric::storage::TemplateVault;
use secure_biometric::templates::{Template, TemplateMetadata, TemplateType};
use tempfile::TempDir;

async fn benchmark_template_storage() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let vault = TemplateVault::new(temp_dir.path())
        .await
        .expect("Failed to create vault");

    let template = Template::new(
        vec![1, 2, 3, 4, 5],
        TemplateMetadata {
            version: "1.0".to_string(),
            template_type: TemplateType::Face,
            quality_score: 0.95,
            extra: serde_json::json!({}),
        },
    );

    vault.store(template).await.expect("Failed to store template");
}

fn storage_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("template_storage", |b| {
        b.iter(|| rt.block_on(benchmark_template_storage()));
    });
}

criterion_group!(benches, storage_benchmark);
criterion_main!(benches);
