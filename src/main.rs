use hardware_controller::{
    async_map_sensors, create_real_sdr_stream, log_execution,
    process_radio_stream, process_telemetry_with_timeout, ApiKeyStrategy, AuthProxy,
    CommandQueue, EventBus, EvictionPolicy, HttpRequest, Memoizer, SystemEvent, TelemetryCounter,
};
use std::time::Duration;
use tokio::sync::oneshot;
fn connect_to_hardware(port: &str) -> Result<String, String> {
    if port == "COM1" { Ok("З'єднання встановлено".to_string()) } 
    else { Err(format!("Порт {} недоступний", port)) }
}

fn heavy_dsp_calculation(voltage: u32) -> String {
    std::thread::sleep(Duration::from_millis(500));
    format!("Сигнал відфільтровано: {}mV", voltage * 2)
}

#[tokio::main]
async fn main() {
    println!(" SMART HARDWARE CONTROLLER v1.0");

    // TASK 9: Логування та запуск (Logging Decorator)
    println!("TASK 9: Ініціалізація системи (Логування)");
    let _ = log_execution!("INFO", connect_to_hardware("COM1"));
    let _ = log_execution!("ERROR", connect_to_hardware("COM99")); 
    println!();

    // TASK 8: Авторизація (Authentication Proxy)
    println!("TASK 8: Перевірка прав доступу (Auth Proxy)");
    let api_proxy = AuthProxy::new(Box::new(ApiKeyStrategy {
        api_key: "super_secret_hardware_key_999".to_string(),
    }));
    let request = HttpRequest::new("https://cloud-server.com/api/boot", "{ action: 'start' }");
    let _ = api_proxy.send_request(request);
    println!();

    // TASK 1: Збір телеметрії (Generators and Iterators)
    println!("TASK 1: Збір телеметрії (Генератори)");
    let sensor_generator = TelemetryCounter::new(1000);
    process_telemetry_with_timeout(sensor_generator, 2);
    println!();

    // TASK 3: Кешування обчислень (Memoization Function)
    println!("TASK 3: Обробка сигналів (Memoization)");
    let mut signal_processor = Memoizer::new(heavy_dsp_calculation, 2, EvictionPolicy::Lru);
    signal_processor.call(5);  
    signal_processor.call(10); 
    signal_processor.call(5);  
    println!();

    // TASK 4: Управління залізом (Bi-Directional Priority Queue)
    println!("TASK 4: Диспетчер команд (Priority Queue)");
    let mut queue = CommandQueue::new();
    queue.enqueue("Оновити лог", 1);
    queue.enqueue("ЕКСТРЕНЕ ОХОЛОДЖЕННЯ", 100);
    if let Some(cmd) = queue.dequeue_highest() {
        println!("Виконано найвищий пріоритет: {}", cmd.action);
    }
    println!();

    // TASK 5: Асинхронний моніторинг (Async Array Functions)
    println!("TASK 5: Асинхронний моніторинг датчиків");
    let sensors = vec!["Температура", "Вольтаж", "Тиск"];
    let (cancel_tx, cancel_rx) = oneshot::channel();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(1000)).await;
        let _ = cancel_tx.send(());
    });

    let _ = async_map_sensors(sensors, |sensor| async move {
        tokio::time::sleep(Duration::from_millis(600)).await;
        println!("  Датчик '{}' відповів", sensor);
        sensor
    }, cancel_rx).await;
    println!();

    // TASK 6: Обробка радіоефіру (Large Data Streams)
    println!("TASK 6: Потокова обробка (Streams)");
    let data_stream = Box::pin(create_real_sdr_stream());
    process_radio_stream(data_stream, 3).await;
    println!();

    // TASK 7: Реактивні події (EventBus)
    println!("TASK 7: Система критичних сповіщень (EventBus)");
    let event_bus = EventBus::new(16);
    let mut cooler_receiver = event_bus.subscribe();
    
    tokio::spawn(async move {
        if let Ok(SystemEvent::TemperatureHigh(temp)) = cooler_receiver.recv().await {
            println!("[КУЛЕР] Відчув перегрів ({}°C)! Увімкнено охолодження.", temp);
        }
    });

    tokio::time::sleep(Duration::from_millis(200)).await;
    event_bus.publish(SystemEvent::TemperatureHigh(90.5));
    tokio::time::sleep(Duration::from_millis(200)).await;
    println!("РОБОТУ ЗАВЕРШЕНО");
}