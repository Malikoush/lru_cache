use crate::types::LruCache;
use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::hash::Hash;
use std::io::{self, Write};
use std::path::Path;

pub struct Cache<K, V>
where
    K: Hash + Eq + Clone,
{
    capacity: usize,
    map: HashMap<K, V>,
    order: Vec<K>,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Clone,
{
    /// # Examples
    ///
    /// ```rust
    /// use lru_cache::Cache;
    ///
    /// let cache: Cache<String, String> = Cache::new(3);
    /// ```
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            order: Vec::new(),
        }
    }
    /// Insere
    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.map.insert(key.clone(), value);
            self.touch(&key);
            return;
        }

        // Si cache oplein, on retire le moins récemment utilisé
        if self.order.len() == self.capacity {
            if let Some(lru_key) = self.order.first().cloned() {
                self.order.remove(0);
                self.map.remove(&lru_key);
            }
        }

        // Insere la nouvelle
        self.order.push(key.clone());
        self.map.insert(key.clone(), value);
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.touch(key);
            self.map.get(key)
        } else {
            None
        }
    }

    /// Met à jour la clé comme récemment utilisée.
    fn touch(&mut self, key: &K) {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            self.order.remove(pos);
            self.order.push(key.clone());
        }
    }
}

impl<K, V> LruCache<K, V> for Cache<K, V>
where
    K: Eq + Hash + Clone,
{
    fn put(&mut self, key: K, value: V) {
        self.put(key, value);
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        self.get(key)
    }
}

impl Cache<String, String> {
    /// Crée un cache LRU persistant qui charge ses données depuis un fichier.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lru_cache::Cache;
    ///
    /// let mut cache = Cache::new_persistent(3, "mon_cache.txt").unwrap();
    /// ```
    pub fn new_persistent(capacity: usize, path: &str) -> io::Result<Self> {
        let mut cache = Self::new(capacity);

        if Path::new(path).exists() {
            let content = read_to_string(path)?;
            for line in content.lines() {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].to_string();
                    let value = parts[1].to_string();
                    cache.put(key, value);
                }
            }
        }

        Ok(cache)
    }

    /// Sauvegarde tout le contenu du cache dans un fichier.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use lru_cache::Cache;
    /// # let mut cache: Cache<String, String> = Cache::new(3);
    /// # cache.put("A".to_string(), "1".to_string());
    /// cache.save_to_file("mon_cache.txt").unwrap();
    /// ```
    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        for key in &self.order {
            if let Some(value) = self.map.get(key) {
                writeln!(file, "{}={}", key, value)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache_basic() {
        // Ici V = String
        let mut cache: Cache<String, String> = Cache::new(3);
        cache.put("A".to_string(), String::from("value_a"));
        cache.put("B".to_string(), String::from("value_b"));
        cache.put("C".to_string(), String::from("value_c"));
        cache.put("D".to_string(), String::from("value_d"));
        assert_eq!(cache.get(&"A".to_string()), None);
        assert_eq!(cache.get(&"D".to_string()), Some(&String::from("value_d")));
        assert_eq!(cache.get(&"B".to_string()), Some(&String::from("value_b")));
        assert_eq!(cache.get(&"C".to_string()), Some(&String::from("value_c")));
        assert_eq!(cache.get(&"X".to_string()), None);
        cache.put("A".to_string(), String::from("value_a"));
        cache.put("X".to_string(), String::from("value_x"));
        assert_eq!(cache.get(&"B".to_string()), None);
        assert_eq!(cache.get(&"D".to_string()), None);
    }

    #[test]
    fn test_lru_cache_with_integers() {
        // Ici V = i32
        let mut cache: Cache<String, i32> = Cache::new(2);
        cache.put("one".to_string(), 1);
        cache.put("two".to_string(), 2);
        assert_eq!(cache.get(&"one".to_string()), Some(&1));
        cache.put("three".to_string(), 3);
    }
}
