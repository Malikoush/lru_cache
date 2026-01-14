use lru_cache::Cache;
use std::fs;

#[test]
fn test_persistent_cache_roundtrip() {
    let path = "test_persistent_cache.txt";
    let _ = fs::remove_file(path); // Nettoyer avant

    let mut cache = Cache::new_persistent(2, path).unwrap();
    cache.put("A".to_string(), "valeur_A".to_string());
    cache.put("B".to_string(), "valeur_B".to_string());
    cache.save_to_file(path).unwrap();

    let mut cache2 = Cache::new_persistent(2, path).unwrap();
    assert_eq!(cache2.get(&"A".to_string()), Some(&"valeur_A".to_string()));
    assert_eq!(cache2.get(&"B".to_string()), Some(&"valeur_B".to_string()));

    let content = fs::read_to_string(path).unwrap();
    assert!(content.contains("A=valeur_A"));
    assert!(content.contains("B=valeur_B"));

    fs::remove_file(path).unwrap(); // Nettoyer après
}

#[test]
fn test_persistent_cache_eviction() {
    let path = "test_eviction.txt";
    let _ = fs::remove_file(path);

    // Cache de taille 1
    let mut cache = Cache::new_persistent(1, path).unwrap();
    cache.put("A".to_string(), "1".to_string());
    cache.save_to_file(path).unwrap();

    let mut cache2 = Cache::new_persistent(1, path).unwrap();
    cache2.put("B".to_string(), "2".to_string());
    cache2.save_to_file(path).unwrap();

    let mut cache3 = Cache::new_persistent(1, path).unwrap();
    assert_eq!(cache3.get(&"A".to_string()), None);
    assert_eq!(cache3.get(&"B".to_string()), Some(&"2".to_string()));

    fs::remove_file(path).unwrap();
}
