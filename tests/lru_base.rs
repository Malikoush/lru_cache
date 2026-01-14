use lru_cache::Cache;

#[test]
fn test_lru_cache() {
    let mut cache = Cache::new(3); // K = String, V = String (inféré par la suite)
    cache.put("A".to_string(), String::from("value_a"));
    cache.put("B".to_string(), String::from("value_b"));
    cache.put("C".to_string(), String::from("value_c"));
    cache.put("D".to_string(), String::from("value_d"));

    let my_value = cache.get(&"A".to_string());
    assert_eq!(my_value, None);
    let my_value = cache.get(&"D".to_string());
    assert_eq!(my_value, Some(&String::from("value_d")));

    let my_value = cache.get(&"B".to_string());
    assert_eq!(my_value, Some(&String::from("value_b")));

    let my_value = cache.get(&"C".to_string());
    assert_eq!(my_value, Some(&String::from("value_c")));

    let my_value = cache.get(&"X".to_string());
    assert_eq!(my_value, None);

    cache.put("A".to_string(), String::from("value_a"));
    cache.put("X".to_string(), String::from("value_x"));

    let my_value = cache.get(&"B".to_string());
    assert_eq!(my_value, None);

    let my_value = cache.get(&"D".to_string());
    assert_eq!(my_value, None);
}
