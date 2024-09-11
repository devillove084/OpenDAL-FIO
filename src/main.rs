#![feature(portable_simd)]
use std::sync::{Arc, Mutex};

use anyhow::Ok;
use clap::Parser;
use crossbeam_channel::{unbounded, Receiver, Sender};
use opendal::{services::Fs, Operator};

mod config;
pub use config::*;

mod generate;
pub use generate::*;

mod util;
pub use util::*;

mod execute;
pub use execute::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Configs::parse();

    if !cli.check() {
        panic!("config not check pass!");
    }

    let builder = Fs::default().root(&cli.path());

    let op = Arc::new(Operator::new(builder)?.finish());

    let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = unbounded();
    let data_generator = DataGenerate::new(cli.file_size(), cli.block_size(), false);

    let gen = std::thread::spawn(move || {
        data_generator.generate(tx);
    });

    let metrics = Arc::new(Mutex::new(MetricsCollector::new(1)));

    let runner = Runner::new(cli.num_jobs(), op, cli.path(), metrics.clone());
    let runner_task = tokio::spawn(async move { runner.start(rx).await });

    let metrics_clone = metrics.clone();
    let metric_task = std::thread::spawn(move || loop {
        {
            let metrics = metrics_clone.lock().unwrap();
            if metrics.is_done() {
                break;
            }
            metrics.display_metrics();
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    });

    gen.join().unwrap();
    runner_task.await.unwrap();
    metric_task.join().unwrap();

    let metrics = metrics.lock().unwrap();
    metrics.display_metrics();

    Ok(())
}
