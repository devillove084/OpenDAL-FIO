use crossterm::{cursor, execute, terminal};
use std::io::Write;
use std::sync::Mutex;
use std::{io::stdout, time::Instant};

pub struct MetricsCollector {
    start_time: Instant,
    total_bytes_written: u64,
    total_operations: u64,
    is_completed: Mutex<bool>,
    job_id: usize,
}

impl MetricsCollector {
    pub fn new(job_id: usize) -> Self {
        MetricsCollector {
            start_time: Instant::now(),
            total_bytes_written: 0,
            total_operations: 0,
            is_completed: Mutex::new(false),
            job_id,
        }
    }

    pub fn mark_completed(&self) {
        let mut completed = self.is_completed.lock().unwrap();
        *completed = true;
    }

    pub fn is_done(&self) -> bool {
        let completed = self.is_completed.lock().unwrap();
        *completed
    }

    pub fn record_write(&mut self, bytes_written: u64) {
        self.total_bytes_written += bytes_written;
        self.total_operations += 1;
    }

    pub fn display_metrics(&self) {
        let elapsed = self.start_time.elapsed();
        let seconds = elapsed.as_secs_f64();

        let iops = self.total_operations as f64 / seconds;
        let bandwidth = (self.total_bytes_written as f64 / 1024.0) / seconds;

        let mut stdout = stdout();
        execute!(
            stdout,
            cursor::MoveUp(1),
            terminal::Clear(terminal::ClearType::CurrentLine),
        )
        .unwrap();

        writeln!(
            stdout,
            "Job {}: IOPS={:.2}, BW={:.2} KiB/s, TotalBytesWritten={} KiB, TimeElapsed={:.2} seconds",
            self.job_id, iops, bandwidth, self.total_bytes_written / 1024, seconds
        ).unwrap();
    }
}
