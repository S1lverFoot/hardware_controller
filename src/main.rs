use hardware_controller::{TelemetryCounter, process_telemetry_with_timeout, Memoizer, EvictionPolicy};
use std::thread;
use std::time::Duration;

fn heavy_dsp_calculation(signal_voltage: u32) -> String {
    thread::sleep(Duration::from_secs(1)); 
    format!("Оброблений сигнал: {}mV", signal_voltage * 2)
}

fn main() {
    println!("--- ЗАПУСК СИСТЕМИ (TASK 1) ---");
    let sensor_generator = TelemetryCounter::new(1000);
    process_telemetry_with_timeout(sensor_generator, 2);

    println!("\n--- ТЕСТ КЕШУВАННЯ (TASK 3) ---");
    
    let mut signal_processor = Memoizer::new(
        heavy_dsp_calculation, 
        2, 
        EvictionPolicy::Lru
    );

    signal_processor.call(5);
    
    signal_processor.call(10);
    
    signal_processor.call(5);

    signal_processor.call(15);
}