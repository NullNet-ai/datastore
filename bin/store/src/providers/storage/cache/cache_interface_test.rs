#[cfg(test)]
mod tests {
    use super::super::cache_interface::CacheInterface;
    use super::super::in_memory_cache::InMemoryCache;
    use std::time::Duration;

    #[test]
    fn test_cache_interface_basic_operations() {
        let cache: Box<dyn CacheInterface<String, i32>> = Box::new(InMemoryCache::new());
        
        // Test initial state
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
        assert!(!cache.contains_key(&"key1".to_string()));
        assert_eq!(cache.get(&"key1".to_string()), None);
        
        // Test insert and get
        cache.insert("key1".to_string(), 42);
        assert!(!cache.is_empty());
        assert_eq!(cache.len(), 1);
        assert!(cache.contains_key(&"key1".to_string()));
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        
        // Test multiple inserts
        cache.insert("key2".to_string(), 100);
        cache.insert("key3".to_string(), 200);
        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(&"key2".to_string()), Some(100));
        assert_eq!(cache.get(&"key3".to_string()), Some(200));
        
        // Test remove
        let removed = cache.remove(&"key2".to_string());
        assert_eq!(removed, Some(100));
        assert_eq!(cache.len(), 2);
        assert!(!cache.contains_key(&"key2".to_string()));
        assert_eq!(cache.get(&"key2".to_string()), None);
        
        // Test remove non-existent key
        let removed = cache.remove(&"nonexistent".to_string());
        assert_eq!(removed, None);
        assert_eq!(cache.len(), 2);
        
        // Test clear
        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
        assert!(!cache.contains_key(&"key1".to_string()));
        assert!(!cache.contains_key(&"key3".to_string()));
    }
    
    #[test]
    fn test_cache_interface_with_ttl() {
        let cache: Box<dyn CacheInterface<String, String>> = Box::new(InMemoryCache::new());
        
        // Test insert with TTL (note: InMemoryCache doesn't actually implement TTL)
        cache.insert_with_ttl(
            "ttl_key".to_string(), 
            "ttl_value".to_string(), 
            Duration::from_secs(60)
        );
        
        assert!(cache.contains_key(&"ttl_key".to_string()));
        assert_eq!(cache.get(&"ttl_key".to_string()), Some("ttl_value".to_string()));
        assert_eq!(cache.len(), 1);
    }
    
    #[test]
    fn test_cache_interface_overwrite_values() {
        let cache: Box<dyn CacheInterface<i32, String>> = Box::new(InMemoryCache::new());
        
        // Insert initial value
        cache.insert(1, "first".to_string());
        assert_eq!(cache.get(&1), Some("first".to_string()));
        assert_eq!(cache.len(), 1);
        
        // Overwrite with new value
        cache.insert(1, "second".to_string());
        assert_eq!(cache.get(&1), Some("second".to_string()));
        assert_eq!(cache.len(), 1); // Length should remain the same
    }
    
    #[test]
    fn test_cache_interface_different_types() {
        let cache: Box<dyn CacheInterface<String, Vec<i32>>> = Box::new(InMemoryCache::new());
        
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![4, 5, 6];
        
        cache.insert("vec1".to_string(), vec1.clone());
        cache.insert("vec2".to_string(), vec2.clone());
        
        assert_eq!(cache.get(&"vec1".to_string()), Some(vec1));
        assert_eq!(cache.get(&"vec2".to_string()), Some(vec2));
        assert_eq!(cache.len(), 2);
    }
}