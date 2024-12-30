mod metrics;

use secure_biometric::logging;
pub use metrics::{TestMetrics, TestTimer};
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use chrono;

/// Test utilities and common functionality
pub struct TestContext {
    pub temp_dir: TempDir,
    pub metrics: Arc<TestMetrics>,
}

impl TestContext {
    /// Create a new test context with temporary directory and metrics
    pub fn new() -> Self {
        // Initialize logging only once
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            Builder::new()
                .format(|buf, record| {
                    writeln!(buf,
                        "{} [{}] {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                        record.level(),
                        record.args()
                    )
                })
                .filter(None, LevelFilter::Info) // Set default to Info
                .filter(Some("sled"), LevelFilter::Warn) // Reduce sled noise
                .init();
        });

        Self {
            temp_dir: TempDir::new().expect("Failed to create temp directory"),
            metrics: TestMetrics::new(),
        }
    }

    /// Get path to temporary directory
    pub fn temp_path(&self) -> PathBuf {
        self.temp_dir.path().to_path_buf()
    }

    /// Create test template data
    pub fn create_test_template(&self) -> Vec<u8> {
        // TODO: Implement realistic template generation
        vec![1, 2, 3, 4, 5]
    }
    
    /// Create a new test timer
    pub fn timer(&self, name: &str) -> TestTimer {
        TestTimer::new(name, self.metrics.clone())
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
