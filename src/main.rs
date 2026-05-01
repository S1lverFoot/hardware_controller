use hardware_controller::{EventBus, SystemEvent};
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("ТЕСТ РЕАКТИВНОЇ ВЗАЄМОДІЇ (TASK 7)\n");
    let event_bus = EventBus::new(16);
    let mut cooler_receiver = event_bus.subscribe();
    
    tokio::spawn(async move {
        while let Ok(event) = cooler_receiver.recv().await {
            match event {
                SystemEvent::TemperatureHigh(temp) => {
                    println!("[КУЛЕР] Відчув перегрів ({}°C)! Вмикаю вентилятори на 100%.", temp);
                }
                SystemEvent::SystemShutdown => {
                    println!("[КУЛЕР] Отримав наказ на вимкнення. Зупиняю обертання.");
                    break; 
                }
                _ => {} 
            }
        }
        println!("[КУЛЕР] Відключився від шини подій.");
    });

    let mut display_receiver = event_bus.subscribe();
    
    tokio::spawn(async move {
        while let Ok(event) = display_receiver.recv().await {
            match event {
                SystemEvent::BatteryLow(percent) => {
                    println!("[ДИСПЛЕЙ] Малює червону іконку: Батарея розряджена! Залишилося {}%.", percent);
                }
                SystemEvent::SystemShutdown => {
                    println!("[ДИСПЛЕЙ] Прощавай! Гашу екран.");
                    break; 
                }
                _ => {}
            }
        }
        println!("[ДИСПЛЕЙ] Відключився від шини подій.");
    });

    println!("Система активована. Датчики починають публікувати події...\n");
    tokio::time::sleep(Duration::from_millis(500)).await;
    event_bus.publish(SystemEvent::TemperatureHigh(85.5));
    tokio::time::sleep(Duration::from_millis(500)).await;
    event_bus.publish(SystemEvent::BatteryLow(15));
    tokio::time::sleep(Duration::from_millis(500)).await;
    event_bus.publish(SystemEvent::SystemShutdown);
    tokio::time::sleep(Duration::from_millis(200)).await;

    println!("\nТест реактивної системи успішно завершено!");
}