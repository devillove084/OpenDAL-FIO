use crossbeam_channel::Receiver;
use opendal::Operator;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{runtime::Runtime, time::sleep};

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

    pub fn start(&self, rx: Receiver<Vec<u8>>) {
        let operator = self.operator.clone();
        let num_jobs = self.num_jobs;

        let rt = Runtime::new().unwrap();

        for _ in 0..num_jobs {
            let operator_clone = operator.clone();
            let output_path_clone = self.path.clone();
            let rx_clone = rx.clone();
            let metrics_clone = self.metrics.clone();

            rt.spawn(async move {
                while let Some(data_chunk) = rx_clone.recv().ok() {
                    let operator_clone = operator_clone.clone();
                    let output_path_clone = output_path_clone.clone();
                    let size = data_chunk.len() as u64;
                    let metrics_inner_clone = metrics_clone.clone();

                    let task = tokio::spawn(async move {
                        let res = operator_clone.write(&output_path_clone, data_chunk).await;
                        if let Err(e) = res {
                            eprintln!("Failed to write chunk: {:?}", e);
                        } else {
                            let mut metrics = metrics_inner_clone.lock().unwrap();
                            metrics.record_write(size);
                        }
                    });

                    task.await.unwrap();
                }
            });

            let metrics_clone = self.metrics.clone();
            rt.spawn(async move {
                loop {
                    {
                        let metrics = metrics_clone.lock().unwrap();
                        metrics.display_metrics();
                    }
                    sleep(Duration::from_secs(1)).await;
                }
            });
        }
    }
}
