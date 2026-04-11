use hardware_controller::{TelemetryCounter, process_telemetry_with_timeout};

fn main() {
    println!("Запуск Smart Hardware Controller...\n");
    let sensor_generator = TelemetryCounter::new(1000);
    process_telemetry_with_timeout(sensor_generator, 3);
    println!("\nРоботу системи успішно завершено.");
}