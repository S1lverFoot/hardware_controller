use hardware_controller::CommandQueue;

fn main() {
    println!("ТЕСТУВАННЯ ЧЕРГИ КОМАНД (TASK 4)\n");

    let mut controller_queue = CommandQueue::new();

    controller_queue.enqueue("Блимати зеленим світлодіодом", 1);    // Низький пріоритет (додано першим - oldest)
    controller_queue.enqueue("Оновити прошивку по Wi-Fi", 5);       // Середній пріоритет
    controller_queue.enqueue("ЕКСТРЕНА ЗУПИНКА ЖИВЛЕННЯ!", 100);    // Супер високий пріоритет
    controller_queue.enqueue("Перевірити статус батареї", 2);       // Низький пріоритет (додано останнім - newest)

    println!("\n--- ПОЧИНАЄМО ВИКОНАННЯ КОМАНД ---");

    if let Some(cmd) = controller_queue.dequeue_highest() {
        println!("Виконую Highest Priority: {}", cmd.action);
    }

    if let Some(cmd) = controller_queue.dequeue_lowest() {
        println!("Виконую Lowest Priority: {}", cmd.action);
    }

    if let Some(cmd) = controller_queue.dequeue_newest() {
        println!("Виконую Newest (останню додану): {}", cmd.action);
    }

    if let Some(cmd) = controller_queue.dequeue_oldest() {
        println!("Виконую Oldest (найдавнішу): {}", cmd.action);
    }
}