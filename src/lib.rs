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