use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkReport {
    pub name: String,
    pub samples: u32,
    pub min_us: f64,
    pub max_us: f64,
    pub avg_us: f64,
    pub p50_us: f64,
    pub p95_us: f64,
    pub p99_us: f64,
    pub throughput_mbps: Option<f64>,
}

impl BenchmarkReport {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn to_human(&self) -> String {
        let mut s = format!("Benchmark: {}\n", self.name);
        s.push_str(&format!("  Samples:   {}\n", self.samples));
        s.push_str(&format!("  Min:       {:.2} µs\n", self.min_us));
        s.push_str(&format!("  Max:       {:.2} µs\n", self.max_us));
        s.push_str(&format!("  Avg:       {:.2} µs\n", self.avg_us));
        s.push_str(&format!("  P50:       {:.2} µs\n", self.p50_us));
        s.push_str(&format!("  P95:       {:.2} µs\n", self.p95_us));
        s.push_str(&format!("  P99:       {:.2} µs\n", self.p99_us));
        if let Some(tp) = self.throughput_mbps {
            s.push_str(&format!("  Throughput: {:.2} MB/s\n", tp));
        }
        s
    }
}
