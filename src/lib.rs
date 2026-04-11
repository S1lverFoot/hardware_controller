use std::time::{Duration, Instant};
use std::thread;

pub struct TelemetryCounter {
    current_id: u64,
}

impl TelemetryCounter {
    pub fn new(start: u64) -> Self {
        TelemetryCounter { current_id: start }
    }
}

impl Iterator for TelemetryCounter {
    type Item = u64; 

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current_id;
        self.current_id += 1;

        Some(value) 
    }
}

pub fn process_telemetry_with_timeout<I>(iterator: I, timeout_secs: u64) 
where
    I: Iterator<Item = u64>,
{
    let timeout = Duration::from_secs(timeout_secs);
    let start_time = Instant::now(); 

    println!("--- Розпочато збір телеметрії (Таймаут: {} сек) ---", timeout_secs);

    for packet_id in iterator {
        if start_time.elapsed() >= timeout {
            println!("--- Таймаут досягнуто! Збір даних зупинено. ---");
        }
        if start_time.elapsed() >= timeout {
            println!("--- Таймаут досягнуто! Збір даних зупинено. ---");
                break; 
}
        println!("Оброблено пакет телеметрії ID: {}", packet_id);
        thread::sleep(Duration::from_millis(500));
    }
}