use hardware_controller::{
    process_telemetry_with_timeout, CommandQueue, EvictionPolicy, Memoizer, TelemetryCounter,
    async_map_sensors, map_sensors_callback,
};
use std::time::Duration;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    println!("\n--- ТЕСТ АСИНХРОННОГО ОПИТУВАННЯ (TASK 5) ---");
    let sensors = vec!["Температура", "Вольтаж", "Тиск", "Оберти кулера"];
    println!(">> Запуск Callback-варіанту:");
    map_sensors_callback(
        sensors.clone(),
        |sensor| format!("{} = OK", sensor), 
        |results| println!("Callback результати: {:?}", results), 
    );

    println!("\n>> Запуск Async-варіанту з підтримкою скасування:");
    let (cancel_tx, cancel_rx) = oneshot::channel();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(2000)).await;
        let _ = cancel_tx.send(()); 
    });

    println!("Починаємо опитування датчиків (забере 4 секунди)...");
    let result = async_map_sensors(
        sensors,
        |sensor| async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("   Зчитано: {}", sensor);
            format!("{} = 100%", sensor)
        },
        cancel_rx, 
    ).await;

    match result {
        Some(data) => println!("Успішно зібрано: {:?}", data),
        None => println!("ПОМИЛКА: Опитування було перервано через тайм-аут!"),
    }
}