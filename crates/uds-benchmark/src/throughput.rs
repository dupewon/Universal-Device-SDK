use std::time::Instant;

pub struct ThroughputBenchmark;

impl ThroughputBenchmark {
    pub fn measure<F>(size_bytes: usize, mut f: F) -> f64
    where
        F: FnMut(),
    {
        let start = Instant::now();
        f();
        let elapsed = start.elapsed().as_secs_f64();
        size_bytes as f64 / elapsed / 1024.0 / 1024.0
    }
}
