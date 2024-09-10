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
    let metrics = Arc::new(Mutex::new(MetricsCollector::new(1)));

    let data_generator = DataGenerate::new(cli.file_size(), cli.block_size(), false);

    let runner = Runner::new(8, op, cli.path(), metrics.clone());

    tokio::spawn(async move { data_generator.generate(tx) });

    runner.start(rx);

    Ok(())
}
