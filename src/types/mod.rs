/// # Examples
///
/// ```rust
/// use lru_cache::{Cache, types::LruCache};
///
/// let mut cache: Cache<String, String> = Cache::new(2);
/// LruCache::put(&mut cache, "A".to_string(), "1".to_string());
/// assert_eq!(LruCache::get(&mut cache, &"A".to_string()), Some(&"1".to_string()));
/// ```

pub trait LruCache<K, V> {
    /// Insère ou met à jour
    fn put(&mut self, key: K, value: V);

    /// Récupère une référence
    fn get(&mut self, key: &K) -> Option<&V>;
}
