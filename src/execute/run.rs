use crossbeam_channel::Receiver;
use opendal::Operator;
use std::sync::{Arc, Mutex};

use super::MetricsCollector;

pub struct Runner {
    num_jobs: usize,
    // io_depth: usize,
    operator: Arc<Operator>,
    path: String,
    metrics: Arc<Mutex<MetricsCollector>>,
}

impl Runner {
    pub fn new(
        num_jobs: usize,
        // io_depth: usize,
        operator: Arc<Operator>,
        path: String,
        metrics: Arc<Mutex<MetricsCollector>>,
    ) -> Self {
        Runner {
            num_jobs,
            // io_depth,
            operator,
            path,
            metrics,
        }
    }

    pub async fn start(&self, rx: Receiver<Vec<u8>>) {
        let operator = self.operator.clone();
        let num_jobs = self.num_jobs;

        let metrics_clone = self.metrics.clone();
        for _ in 0..num_jobs {
            let operator_clone = operator.clone();
            let output_path_clone = self.path.clone();
            let rx_clone = rx.clone();

            while let Some(data_chunk) = rx_clone.recv().ok() {
                let operator_clone = operator_clone.clone();
                let output_path_clone = output_path_clone.clone();
                let size = data_chunk.len() as u64;

                let res = operator_clone.write(&output_path_clone, data_chunk).await;
                if let Err(e) = res {
                    eprintln!("Failed to write chunk: {:?}", e);
                } else {
                    let mut metrics = metrics_clone.lock().unwrap();
                    metrics.record_write(size);
                }
            }
        }
        let metrics = metrics_clone.lock().unwrap();
        metrics.mark_completed();
    }
}
