# LRU Cache Rust

**Cache LRU générique avec persistance fichier**

## Quick Start

git clone https://github.com/Malikoush/lru_cache.git

cd lru_cache

# Tests

cargo test

# Documentation

cargo doc --open

# Benchmarks

cargo run --bin bench

## Tests Coverage

Unitaires : src/cache/lru.rs (2 tests)

Intégration : tests/ (3 tests)

Doctests : 100% exemples compilés

## Features

| Fonctionnalité      | Statut |
| ------------------- | ------ |
| Générique `K,V`     | OK     |
| Trait `LruCache`    | OK     |
| Persistance fichier | OK     |
