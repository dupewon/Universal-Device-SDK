use std::time::{Duration, Instant};

pub struct LatencyBenchmark {
    samples: Vec<Duration>,
}

impl Default for LatencyBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl LatencyBenchmark {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    pub fn measure<F>(&mut self, mut f: F) -> Duration
    where
        F: FnMut(),
    {
        let start = Instant::now();
        f();
        let elapsed = start.elapsed();
        self.samples.push(elapsed);
        elapsed
    }

    pub fn stats(&self) -> BenchmarkStats {
        if self.samples.is_empty() {
            return BenchmarkStats::default();
        }
        let mut sorted = self.samples.clone();
        sorted.sort();
        let total: Duration = sorted.iter().sum();
        BenchmarkStats {
            min: sorted[0],
            max: sorted[sorted.len() - 1],
            avg: total / sorted.len() as u32,
            p50: sorted[sorted.len() / 2],
            p95: sorted[(sorted.len() as f64 * 0.95) as usize],
            p99: sorted[(sorted.len() as f64 * 0.99) as usize],
        }
    }
}

#[derive(Debug, Default)]
pub struct BenchmarkStats {
    pub min: Duration,
    pub max: Duration,
    pub avg: Duration,
    pub p50: Duration,
    pub p95: Duration,
    pub p99: Duration,
}
