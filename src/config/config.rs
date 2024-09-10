use crate::parse_human_readable_size;
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum IOEngine {
    Sync,
    PSync,
    VSync,
    Aio,
    IOUring,
    Remote,
    Rdma,
}

impl Default for IOEngine {
    fn default() -> Self {
        IOEngine::Sync
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum RwType {
    Read,
    Write,
    ReadWrite,
    RandRead,
    RandWrite,
    RandReadWrite,
}

impl Default for RwType {
    fn default() -> Self {
        RwType::Write
    }
}

#[derive(Debug, Parser)]
pub struct Configs {
    #[arg(
        short = 'n',
        long,
        default_value_t = 1,
        help = "Number of jobs to run (default: 1)"
    )]
    num_jobs: usize,

    #[arg(short = 'd', long, default_value_t = 1, help = "IO depth (default: 1)")]
    io_depth: usize,

    #[arg(short = 'e', long, value_enum, default_value_t = IOEngine::default(), help = "IO engine to use (default: Sync)")]
    io_engine: IOEngine,

    #[arg(short = 's', long, default_value_t = 1, value_parser = parse_human_readable_size, help = "File size in human-readable format (e.g., 10KB, 100MB, 1GB, default: 100MB)")]
    file_size: u64,

    #[arg(
        short = 'b',
        long,
        default_value_t = 1,
        help = "Block size (default: 1)"
    )]
    block_size: u64,

    #[arg(short = 't', long, value_enum, default_value_t = RwType::default(), help = "RW type (default: Write)")]
    rw_type: RwType,

    #[arg(short = 'p', long)]
    path: String,
}

impl Configs {
    pub fn check(&self) -> bool {
        self.num_jobs != 0 && self.io_depth != 0 && self.file_size != 0
    }
}

impl Configs {
    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    pub fn block_size(&self) -> u64 {
        self.block_size
    }
}
