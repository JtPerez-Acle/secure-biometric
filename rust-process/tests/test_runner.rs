mod common;

use common::TestMetrics;
use log::info;
use std::sync::Arc;

pub struct TestRunner {
    metrics: Arc<TestMetrics>,
}

impl TestRunner {
    pub fn new() -> Self {
        Self {
            metrics: TestMetrics::new(),
        }
    }

    pub async fn run_all_tests(&self) {
        info!("Starting test suite execution");

        // Run security tests
        self.run_security_tests().await;

        // Print final summary
        self.metrics.print_summary().await;
    }

    async fn run_security_tests(&self) {
        info!("Running security test suite");
        
        // Add test execution here
        // We'll implement this after fixing the encryption tests
    }
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}
