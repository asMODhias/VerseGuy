use crate::prelude::*;
use lru::LruCache;
use std::hash::Hash;
use std::num::NonZeroUsize;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tracing::debug;

/// LRU cache with TTL
pub struct Cache<K, V> {
    cache: Mutex<LruCache<K, CacheEntry<V>>>,
    ttl: Duration,
}

struct CacheEntry<V> {
    value: V,
    inserted_at: Instant,
}

impl<K: Hash + Eq, V: Clone> Cache<K, V> {
    /// Create new cache
    pub fn new(capacity: usize, ttl: Duration) -> AppResult<Self> {
        if capacity == 0 {
            return Err(configuration_err("Capacity must be > 0"));
        }
        // SAFETY: capacity > 0
        let n = unsafe { NonZeroUsize::new_unchecked(capacity) };
        Ok(Self {
            cache: Mutex::new(LruCache::new(n)),
            ttl,
        })
    }

    /// Get value from cache
    pub fn get(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.lock().ok()?;

        if let Some(entry) = cache.get(key) {
            // Check TTL
            if entry.inserted_at.elapsed() < self.ttl {
                metrics::counter!("cache_hits_total", 1);
                return Some(entry.value.clone());
            } else {
                // Expired, remove
                cache.pop(key);
            }
        }

        metrics::counter!("cache_misses_total", 1);
        None
    }

    /// Put value in cache
    pub fn put(&self, key: K, value: V) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.put(
                key,
                CacheEntry {
                    value,
                    inserted_at: Instant::now(),
                },
            );
        }
    }

    /// Invalidate key
    pub fn invalidate(&self, key: &K) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.pop(key);
        }
    }

    /// Clear entire cache
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
            debug!("Cache cleared");
        }
    }

    /// Get cache size
    pub fn len(&self) -> usize {
        self.cache.lock().map_or(0, |c| c.len())
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_get_put() -> AppResult<()> {
        let cache = Cache::new(10, Duration::from_secs(60))?;

        cache.put("key1", "value1");
        assert_eq!(cache.get(&"key1"), Some("value1"));
        assert_eq!(cache.get(&"key2"), None);

        Ok(())
    }

    #[test]
    fn test_cache_ttl() -> AppResult<()> {
        let cache = Cache::new(10, Duration::from_millis(100))?;

        cache.put("key1", "value1");
        assert_eq!(cache.get(&"key1"), Some("value1"));

        std::thread::sleep(Duration::from_millis(150));

        assert_eq!(cache.get(&"key1"), None);
        Ok(())
    }

    #[test]
    fn test_cache_capacity() -> AppResult<()> {
        let cache = Cache::new(2, Duration::from_secs(60))?;

        cache.put("key1", "value1");
        cache.put("key2", "value2");
        cache.put("key3", "value3"); // Should evict key1

        assert_eq!(cache.get(&"key1"), None);
        assert_eq!(cache.get(&"key2"), Some("value2"));
        assert_eq!(cache.get(&"key3"), Some("value3"));
        Ok(())
    }
}
