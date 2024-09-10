use crossbeam_channel::Sender;
use rand_distr::{Distribution, Normal};
use std::simd::Simd;
use std::thread;

pub struct DataGenerate {
    file_size: u64,
    block_size: u64,
    simd_enabled: bool,
}

impl DataGenerate {
    pub fn new(file_size: u64, block_size: u64, simd_enabled: bool) -> Self {
        DataGenerate {
            file_size,
            block_size,
            simd_enabled,
        }
    }

    pub fn generate(&self, tx: Sender<Vec<u8>>) -> std::thread::JoinHandle<()> {
        let block_size = self.block_size;
        let simd_enabled = self.simd_enabled;
        let file_size = self.file_size;

        let normal = Normal::new(0.0, 1.0).unwrap();

        thread::spawn(move || {
            let mut total_written: u64 = 0;
            while total_written < file_size {
                let to_write = std::cmp::min(block_size as u64, file_size - total_written);
                let data_chunk = if simd_enabled {
                    DataGenerate::generate_simd_chunk(to_write as usize)
                } else {
                    DataGenerate::generate_chunk(to_write as usize, &normal)
                };

                tx.send(data_chunk).unwrap();
                total_written += to_write;
            }
        })
    }

    fn generate_chunk(size: usize, normal: &Normal<f64>) -> Vec<u8> {
        let size = 100 * 1024;
        let mut buffer: Vec<u8> = Vec::with_capacity(size);
        for _ in 0..size {
            let value = normal.sample(&mut rand::thread_rng()) as u8;
            buffer.push(value);
        }
        buffer
    }

    fn generate_simd_chunk(size: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size);
        let simd_data: Simd<u8, 8> = Simd::from_array([0; 8]);

        for _ in (0..size).step_by(8) {
            buffer.extend_from_slice(simd_data.as_array());
        }

        buffer
    }
}
