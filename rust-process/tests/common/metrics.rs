use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Default)]
pub struct TestMetrics {
    pub total_tests: AtomicUsize,
    pub passed_tests: AtomicUsize,
    pub failed_tests: AtomicUsize,
    pub total_duration: Mutex<Duration>,
    pub test_durations: Mutex<Vec<(String, Duration)>>,
}

impl TestMetrics {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub async fn record_test(&self, name: &str, duration: Duration, passed: bool) {
        self.total_tests.fetch_add(1, Ordering::SeqCst);
        if passed {
            self.passed_tests.fetch_add(1, Ordering::SeqCst);
        } else {
            self.failed_tests.fetch_add(1, Ordering::SeqCst);
        }

        let mut total_duration = self.total_duration.lock().await;
        *total_duration += duration;

        let mut durations = self.test_durations.lock().await;
        durations.push((name.to_string(), duration));
    }

    pub async fn print_summary(&self) {
        let total = self.total_tests.load(Ordering::SeqCst);
        let passed = self.passed_tests.load(Ordering::SeqCst);
        let failed = self.failed_tests.load(Ordering::SeqCst);
        let total_duration = self.total_duration.lock().await;

        println!("\nTest Summary");
        println!("============");
        println!("Total Tests: {}", total);
        println!("Passed: {}", passed);
        println!("Failed: {}", failed);
        println!("Total Duration: {:?}", *total_duration);

        let durations = self.test_durations.lock().await;
        if !durations.is_empty() {
            println!("\nTest Durations:");
            for (name, duration) in durations.iter() {
                println!("{}: {:?}", name, duration);
            }
        }
    }
}

pub struct TestTimer {
    start: Instant,
    name: String,
    metrics: Arc<TestMetrics>,
}

impl TestTimer {
    pub fn new(name: &str, metrics: Arc<TestMetrics>) -> Self {
        Self {
            start: Instant::now(),
            name: name.to_string(),
            metrics,
        }
    }

    pub async fn stop(self, passed: bool) {
        let duration = self.start.elapsed();
        self.metrics.record_test(&self.name, duration, passed).await;
    }
}
