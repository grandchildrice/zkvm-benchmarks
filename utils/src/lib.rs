use std::{
    fmt::Display,
    fs::File,
    io::Write,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use serde::Serialize;

fn get_current_memory_usage() -> Result<usize, std::io::Error> {
    let content = std::fs::read_to_string("/proc/self/status")?;
    for line in content.lines() {
        if line.starts_with("VmRSS:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(kb) = parts[1].parse::<usize>() {
                    return Ok(kb / 1024); // KB -> MB
                }
            }
        }
    }
    Ok(0)
}

fn measure_peak_memory<R, F: FnOnce() -> R>(func: F) -> (R, usize) {
    let peak = Arc::new(AtomicUsize::new(0));
    let stop = Arc::new(AtomicBool::new(false));

    let peak_clone = Arc::clone(&peak);
    let stop_clone = Arc::clone(&stop);
    let monitor = thread::spawn(move || {
        while !stop_clone.load(Ordering::Relaxed) {
            if let Ok(mem) = get_current_memory_usage() {
                peak_clone.fetch_max(mem, Ordering::Relaxed);
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    let result = func();

    stop.store(true, Ordering::Relaxed);
    monitor.join().unwrap();

    (result, peak.load(Ordering::Relaxed))
}

pub fn benchmark<T: Display + Clone, F>(func: F, inputs: &[T], file: &str, input_name: &str)
where
    F: Fn(T) -> (Duration, usize, usize),
{
    let mut results = Vec::new();
    for input in inputs {
        let ((duration, size, cycles), peak_memory) = measure_peak_memory(|| func(input.clone()));
        results.push((duration, size, cycles, peak_memory));
    }

    write_csv(file, input_name, inputs, &results);
}

pub fn write_csv<T: Display>(
    file: &str,
    _input_name: &str,
    inputs: &[T],
    results: &[(Duration, usize, usize, usize)],
) {
    let mut file = File::create(file).unwrap();
    file.write_all(
        format!("n,cycles,prover time (ms),proof size (bytes),peak memory (MB)\n").as_bytes(),
    )
    .unwrap();
    inputs
        .iter()
        .zip(results)
        .for_each(|(input, (duration, size, cycles, peak_memory))| {
            file.write_all(
                format!(
                    "{},{},{},{},{}\n",
                    input,
                    cycles,
                    duration.as_millis(),
                    size,
                    peak_memory
                )
                .as_bytes(),
            )
            .unwrap();
        });
}

pub fn size<T: Serialize>(item: &T) -> usize {
    bincode::serialized_size(item).unwrap() as usize
}
