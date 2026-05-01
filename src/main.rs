use hardware_controller::{create_radio_data_stream, process_radio_stream};

#[tokio::main]
async fn main() {
    println!("--- ТЕСТ ОБРОБКИ ВЕЛИКИХ ДАНИХ (TASK 6) ---\n");
    println!("Увага: Симуляція обробки гігантського масиву радіоданих...");
    let data_stream = Box::pin(create_radio_data_stream());
    process_radio_stream(data_stream, 5).await;   
    println!("\nОбробку успішно завершено! Оперативна пам'ять комп'ютера не постраждала.");
}