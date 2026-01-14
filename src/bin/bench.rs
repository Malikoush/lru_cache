use lru_cache::Cache;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let n = 10000;
    println!("=== Benchmark {} opérations ===", n);

    // CACHE LRU
    let mut lru = Cache::new(n / 2);
    let start = Instant::now();
    for i in 0..n {
        lru.put(format!("k{}", i), format!("v{}", i));
    }
    let mut hits = 0;
    for i in 0..n {
        if lru.get(&format!("k{}", i % (n / 2))).is_some() {
            hits += 1;
        }
    }
    println!("LRU Cache    : {:?} ({} hits)", start.elapsed(), hits);

    //RECHERCHE LINÉAIRE Vec (lent)
    let mut vec_store: Vec<(String, String)> = Vec::new();
    let start = Instant::now();
    for i in 0..n {
        vec_store.push((format!("k{}", i), format!("v{}", i)));
    }
    let mut hits_vec = 0;
    for i in 0..n {
        for (k, _) in &vec_store {
            if k == &format!("k{}", i % (n / 2)) {
                hits_vec += 1;
                break;
            }
        }
    }
    println!("Vec linéaire : {:?} ({} hits)", start.elapsed(), hits_vec);

    // 3. HashMap référence
    let mut hmap = HashMap::new();
    let start = Instant::now();
    for i in 0..n {
        hmap.insert(format!("k{}", i), format!("v{}", i));
    }
    let mut hits_hmap = 0;
    for i in 0..n {
        if hmap.get(&format!("k{}", i % (n / 2))).is_some() {
            hits_hmap += 1;
        }
    }
    println!("HashMap      : {:?} ({} hits)", start.elapsed(), hits_hmap);
}
