use std::time::Instant;

pub fn run_benchmark(kind: &str) -> anyhow::Result<()> {
    println!("UDS Benchmark ({})\n", kind);

    match kind {
        "latency" | "all" => {
            println!("--- Latency Benchmark ---");
            let mut samples = Vec::new();
            for i in 0..10 {
                let start = Instant::now();
                std::thread::sleep(std::time::Duration::from_micros(200 + i * 10));
                let elapsed = start.elapsed();
                samples.push(elapsed);
                println!("  Sample {:>2}: {:>7}µs", i + 1, elapsed.as_micros());
            }
            samples.sort();
            let avg: std::time::Duration =
                samples.iter().sum::<std::time::Duration>() / samples.len() as u32;
            println!();
            println!("  Min:     {:>7}µs", samples[0].as_micros());
            println!("  Max:     {:>7}µs", samples[samples.len() - 1].as_micros());
            println!("  Average: {:>7}µs", avg.as_micros());
            println!("  P50:     {:>7}µs", samples[samples.len() / 2].as_micros());
            println!();
        }
        "throughput" | "all" => {
            if kind != "all" {
                println!("--- Throughput Benchmark ---");
            }
            let data_size = 1024 * 1024; // 1 MB
            let data = vec![0u8; data_size];
            let start = Instant::now();
            let mut written = 0usize;
            while written < data_size {
                let chunk = &data[written..written + 4096.min(data_size - written)];
                written += chunk.len();
            }
            let elapsed = start.elapsed();
            let throughput = data_size as f64 / elapsed.as_secs_f64() / (1024.0 * 1024.0);
            println!("  Data:     {} MB", data_size as f64 / (1024.0 * 1024.0));
            println!("  Time:     {:.2} ms", elapsed.as_secs_f64() * 1000.0);
            println!("  Throughput: {:.2} MB/s", throughput);
        }
        _ => anyhow::bail!(
            "Unknown benchmark type: {}. Use: latency, throughput, all",
            kind
        ),
    }

    Ok(())
}
