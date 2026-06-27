use std::sync::{Arc, Mutex};
use tokio::time::{Duration, Instant, sleep};
// use tokio::sync::Mutex;

#[derive(Default)]
struct Metrics {
    total_requests: u32,
    total_errors: u32,
    processing_times: Vec<u64>,
}

impl Metrics {
    fn new(total_requests: u32, total_errors: u32, processing_times: Vec<u64>) -> Metrics {
        Metrics {
            total_requests: 0,
            total_errors: 0,
            processing_times: Vec::new(),
        }
    }
}

#[tokio::main]
async fn main() {
    let data1 = Arc::new(Mutex::new(Metrics::default()));

    let mut handles = Vec::new();

    for i in 0..5 {
        let data2 = Arc::clone(&data1);

        handles.push(tokio::spawn(async move {
            let now = Instant::now();
            sleep(Duration::from_millis(300)).await;

            let mut guard = data2.lock().unwrap();

            guard.total_requests += 1;

            if i % 2 == 0 {
                guard.total_errors += 1
            }

            guard
                .processing_times
                .push(now.elapsed().as_millis() as u64);
        }))
    }

    for h in handles {
        h.await.unwrap();
    }

    let metrics = data1.lock().unwrap();

    let total_requests = metrics.total_requests;
    let total_errors = metrics.total_errors;

    let sum: u64 = metrics.processing_times.iter().sum();
    let count = metrics.processing_times.len() as u64;

    let avg = if count == 0 {
        0.0
    } else {
        sum as f64 / count as f64
    };

    let max = metrics.processing_times.iter().max().copied().unwrap_or(0);

    println!("=== METRICS SUMMARY ===");
    println!("Total requests: {}", total_requests);
    println!("Total errors: {}", total_errors);
    println!("Average processing time: {:.2} ms", avg);
    println!("Max processing time: {} ms", max);
}
