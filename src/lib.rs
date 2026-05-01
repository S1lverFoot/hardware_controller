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

    println!("Розпочато збір телеметрії (Таймаут: {} сек)", timeout_secs);

    for packet_id in iterator {
        if start_time.elapsed() >= timeout {
            println!("Таймаут досягнуто. Збір даних зупинено.");
        }
        if start_time.elapsed() >= timeout {
            println!("Таймаут досягнуто. Збір даних зупинено.");
                break; 
}
        println!("Оброблено пакет телеметрії ID: {}", packet_id);
        thread::sleep(Duration::from_millis(500));
    }
}

use std::collections::HashMap;
use std::hash::Hash;

pub enum EvictionPolicy {
    Unlimited, 
    Lru,      
}

pub struct Memoizer<K, V> {
    cache: HashMap<K, V>,    
    history: Vec<K>,         
    max_size: usize,          
    policy: EvictionPolicy,   
    func: fn(K) -> V,          
}

impl<K, V> Memoizer<K, V>
where
    K: Eq + Hash + Clone + std::fmt::Display, 
    V: Clone,
{
    pub fn new(func: fn(K) -> V, max_size: usize, policy: EvictionPolicy) -> Self {
        Self {
            cache: HashMap::new(),
            history: Vec::new(),
            max_size,
            policy,
            func,
        }
    }

    pub fn call(&mut self, arg: K) -> V {
        if let Some(result) = self.cache.get(&arg) {
            println!("[CACHE HIT] Значення для {} знайдено в пам'яті!", arg);
            
            if matches!(self.policy, EvictionPolicy::Lru) {
                self.history.retain(|k| k != &arg);
                self.history.push(arg.clone());
            }
            return result.clone();
        }

        println!("[CACHE MISS] Важкі обчислення для {}...", arg);
        let result = (self.func)(arg.clone());

        if matches!(self.policy, EvictionPolicy::Lru) && self.cache.len() >= self.max_size {
            if let Some(oldest_key) = self.history.first().cloned() {
                println!("[CACHE FULL] Видаляємо найстаріший запис: {}", oldest_key);
                self.cache.remove(&oldest_key);
                self.history.remove(0);
            }
        }

        self.cache.insert(arg.clone(), result.clone());
        self.history.push(arg);

        result
    }
}

#[derive(Debug, Clone)]
pub struct HardwareCommand {
    pub action: String,
    pub priority: i32, 
    pub order_id: u64, 
}

pub struct CommandQueue {
    queue: Vec<HardwareCommand>,
    global_counter: u64,
}

impl CommandQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            global_counter: 0,
        }
    }

    pub fn enqueue(&mut self, action: &str, priority: i32) {
        let cmd = HardwareCommand {
            action: action.to_string(),
            priority,
            order_id: self.global_counter,
        };
        self.queue.push(cmd);
        self.global_counter += 1;
        println!("[ЧЕРГА] Додано команду: '{}' (Пріоритет: {})", action, priority);
    }

    fn dequeue_by<F>(&mut self, mut compare: F) -> Option<HardwareCommand>
    where
        F: FnMut(&HardwareCommand, &HardwareCommand) -> std::cmp::Ordering,
    {
        if self.queue.is_empty() {
            return None;
        }
        
        let index = self
            .queue
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| compare(a, b))
            .map(|(i, _)| i)?;

        Some(self.queue.remove(index))
    }

    pub fn dequeue_highest(&mut self) -> Option<HardwareCommand> {
        self.dequeue_by(|a, b| a.priority.cmp(&b.priority))
    }

    pub fn dequeue_lowest(&mut self) -> Option<HardwareCommand> {
        self.dequeue_by(|a, b| b.priority.cmp(&a.priority))
    }

    pub fn dequeue_oldest(&mut self) -> Option<HardwareCommand> {
        self.dequeue_by(|a, b| b.order_id.cmp(&a.order_id))
    }

    pub fn dequeue_newest(&mut self) -> Option<HardwareCommand> {
        self.dequeue_by(|a, b| a.order_id.cmp(&b.order_id))
    }
}

use std::future::Future;
pub fn map_sensors_callback<T, R, F, C>(data: Vec<T>, mut transform: F, mut on_complete: C)
where
    F: FnMut(T) -> R,
    C: FnMut(Vec<R>),
{
    let mut results = Vec::new();
    for item in data {
        results.push(transform(item));
    }
    on_complete(results);
}

pub async fn async_map_sensors<T, R, F, Fut>(
    data: Vec<T>,
    transform: F,
    mut cancel_rx: tokio::sync::oneshot::Receiver<()>,
) -> Option<Vec<R>>
where
    F: Fn(T) -> Fut,
    Fut: Future<Output = R>,
{
    let mut results = Vec::new();

    for item in data {
        tokio::select! {
            res = transform(item) => {
                results.push(res);
            }
            _ = &mut cancel_rx => {
                println!("[АБОРТ] Сигнал скасування отримано! Зупиняємо опитування.");
                return None;
            }
        } 
    } 

    Some(results)
} 

use tokio_stream::{Stream, StreamExt};
pub fn create_radio_data_stream() -> impl Stream<Item = Vec<u8>> {
    tokio_stream::iter(1..=1_000_000).map(|chunk_number| {
        let fake_iq_data = vec![(chunk_number % 255) as u8; 1024];
        fake_iq_data
    })
}

pub async fn process_radio_stream<S>(mut stream: S, demo_limit: usize) 
where
    S: Stream<Item = Vec<u8>> + std::marker::Unpin,
{
    let mut chunks_processed = 0;
    let mut total_bytes = 0;
    while let Some(chunk) = stream.next().await {
        chunks_processed += 1;
        total_bytes += chunk.len();
        println!("[STREAM] Завантажено чанк №{}. Розмір: {} байт. Загалом оброблено: {} байт", 
                 chunks_processed, chunk.len(), total_bytes);
        tokio::time::sleep(std::time::Duration::from_millis(400)).await;
        if chunks_processed >= demo_limit {
            println!("[STREAM] Досягнуто ліміт демо-обробки. Зупиняємось.");
            break;
        }
    }
}

use tokio::sync::broadcast;
#[derive(Debug, Clone)]
pub enum SystemEvent {
    BatteryLow(u8),     
    TemperatureHigh(f32), 
    SystemShutdown,    
}

pub struct EventBus {
    sender: broadcast::Sender<SystemEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _receiver) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SystemEvent> {
        self.sender.subscribe()
    }

    pub fn publish(&self, event: SystemEvent) {
        let _ = self.sender.send(event);
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn new(url: &str, body: &str) -> Self {
        Self {
            url: url.to_string(),
            headers: HashMap::new(),
            body: body.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub data: String,
}

pub trait AuthStrategy {
    fn apply_auth(&self, request: &mut HttpRequest) -> Result<(), String>;
}

pub struct ApiKeyStrategy {
    pub api_key: String,
}
impl AuthStrategy for ApiKeyStrategy {
    fn apply_auth(&self, request: &mut HttpRequest) -> Result<(), String> {
        request.headers.insert("X-API-KEY".to_string(), self.api_key.clone());
        Ok(())
    }
}

pub struct JwtStrategy {
    pub token: String,
}
impl AuthStrategy for JwtStrategy {
    fn apply_auth(&self, request: &mut HttpRequest) -> Result<(), String> {
        if self.token.is_empty() {
            return Err("JWT токен відсутній або прострочений!".to_string());
        }
        request.headers.insert("Authorization".to_string(), format!("Bearer {}", self.token));
        Ok(())
    }
}

pub struct AuthProxy {
    strategy: Box<dyn AuthStrategy>, 
}

impl AuthProxy {
    pub fn new(strategy: Box<dyn AuthStrategy>) -> Self {
        Self { strategy }
    }

    pub fn set_strategy(&mut self, new_strategy: Box<dyn AuthStrategy>) {
        println!("[PROXY] Стратегію авторизації змінено!");
        self.strategy = new_strategy;
    }

    pub fn send_request(&self, mut request: HttpRequest) -> HttpResponse {
        println!("\n[PROXY] Перехоплено запит до: {}", request.url);
        match self.strategy.apply_auth(&mut request) {
            Ok(_) => {
                println!("[PROXY] Авторизацію успішно додано, заголовки: {:?}", request.headers);
                println!("[СЕРВЕР] Отримано дані: {}. Зберігаю...", request.body);
                HttpResponse { status: 200, data: "Дані збережено успішно".to_string() }
            }
            Err(e) => {
                println!("[PROXY ERROR] Відмовлено в доступі: {}", e);
                HttpResponse { status: 401, data: "Unauthorized".to_string() }
            }
        }
    }
}