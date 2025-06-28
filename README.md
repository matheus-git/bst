# Bst Hashmap

![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

A binary tree implementation.

**Binary trees are not recommended for production since they are not balanced. This repository is intended for study purposes only.**

```rust
let mut map = BstHashmap::<i32, String>::default();

map.insert(5, "five".to_string());
map.insert(3, "three".to_string());
map.insert(7, "seven".to_string());
map.insert(6, "six".to_string());
map.insert(8, "eight".to_string());

if let Some(value) = map.search(7) {
    println!("Found key 7 with value: {}", value);
} else {
    println!("Key 7 not found");
}

if let Some((min_key, min_val)) = map.min(5) {
    println!("Min in subtree of 5: key = {}, value = {}", min_key, min_val);
}

if let Some((max_key, max_val)) = map.max(5) {
    println!("Max in subtree of 5: key = {}, value = {}", max_key, max_val);
}

map.remove(7);
println!("Removed key 7");
 
if map.search(7).is_none() {
    println!("Key 7 no longer found after removal");
}
```

## 📝 License

This project is open-source under the MIT License.
