use crossterm::{cursor, execute, terminal};
use std::io::Write;
use std::{io::stdout, time::Instant};
pub struct MetricsCollector {
    start_time: Instant,
    total_bytes_written: u64,
    total_operations: u64,
    job_id: usize,
}

impl MetricsCollector {
    pub fn new(job_id: usize) -> Self {
        MetricsCollector {
            start_time: Instant::now(),
            total_bytes_written: 0,
            total_operations: 0,
            job_id,
        }
    }

    pub fn record_write(&mut self, bytes_written: u64) {
        self.total_bytes_written += bytes_written;
        self.total_operations += 1;
    }

    pub fn display_metrics(&self) {
        let elapsed = self.start_time.elapsed();
        let seconds = elapsed.as_secs_f64();

        let iops = self.total_operations as f64 / seconds;
        let bandwidth = (self.total_bytes_written as f64 / 1024.0) / seconds; // 带宽单位为 KiB/s

        // 清空并移动光标到终端顶部位置
        let mut stdout = stdout();
        execute!(
            stdout,
            cursor::MoveTo(0, 0),                              // 移动光标到行首
            terminal::Clear(terminal::ClearType::CurrentLine), // 清除当前行
        )
        .unwrap();

        writeln!(
            stdout,
            "Job {}: IOPS={:.2}, BW={:.2} KiB/s, TotalBytesWritten={} KiB, TimeElapsed={:.2} seconds",
            self.job_id, iops, bandwidth, self.total_bytes_written / 1024, seconds
        ).unwrap();
    }
}
