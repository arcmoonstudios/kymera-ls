//! Utility functions shared across the Kymera ecosystem.

use parking_lot::RwLock;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Thread-safe cache with automatic expiration
pub struct Cache<K, V> {
    inner: Arc<RwLock<dashmap::DashMap<K, (V, Instant)>>>,
    ttl: Duration,
}

impl<K: std::hash::Hash + Eq, V> Cache<K, V> {
    pub fn new(ttl: Duration) -> Self {
        Self {
            inner: Arc::new(RwLock::new(dashmap::DashMap::new())),
            ttl,
        }
    }

    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        let map = self.inner.read();
        map.get(key).and_then(|entry| {
            let (value, instant) = entry.value();
            if instant.elapsed() < self.ttl {
                Some(value.clone())
            } else {
                None
            }
        })
    }

    pub fn insert(&self, key: K, value: V)
    where
        K: Clone,
        V: Clone,
    {
        let map = self.inner.write();
        map.insert(key, (value, Instant::now()));
    }

    pub fn remove(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        let map = self.inner.write();
        map.remove(key).map(|(_k, (v, _))| v)
    }

    pub fn clear(&self) {
        let map = self.inner.write();
        map.clear();
    }
}

/// Measure execution time of a function
pub fn measure_time<F, T>(f: F) -> (T, Duration)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_cache() {
        let cache = Cache::new(Duration::from_millis(100));
        cache.insert("key", "value");
        assert_eq!(cache.get(&"key"), Some("value"));
        thread::sleep(Duration::from_millis(150));
        assert_eq!(cache.get(&"key"), None);
    }

    #[test]
    fn test_measure_time() {
        let (result, duration) = measure_time(|| {
            thread::sleep(Duration::from_millis(10));
            42
        });
        assert_eq!(result, 42);
        assert!(duration >= Duration::from_millis(10));
    }
} 