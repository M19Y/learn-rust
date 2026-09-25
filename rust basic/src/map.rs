/* Map
*
* Map is a collection type that contains key-value
* Map use key as an identifer, because its unique every single data
*
* Rust provide 2 types of map:
* 1. HashMap
* 2. BTreeMap
*/

fn main() {
    println!("Map");
}

// 1. HashMap
// Hash map stored data in random order, it became faster when get and insert data
use std::collections::HashMap;
#[test]
fn map_hash_map() {
    let mut map = HashMap::<String, String>::new();

    map.insert(String::from("name"), String::from("Otong"));
    map.insert(String::from("age"), String::from("25"));

    let name = map.get("name");
    let age = map.get("age");

    println!("Name: {}", name.unwrap());
    println!("Age: {}", age.unwrap());
}

// 1. BTreeMap
// BTreeMap stored data in asc order, it became slower than HashMap
use std::collections::BTreeMap;
#[test]
fn map_btree_map() {
    let mut map = BTreeMap::<String, String>::new();

    map.insert(String::from("name"), String::from("Otong"));
    map.insert(String::from("age"), String::from("25"));
    map.insert(String::from("country"), String::from("Wkwk Land"));

    // tuple
    for entry in &map {
        println!("{} : {}", entry.0, entry.1);
    }

    // destructuring tuple
    for (key, value) in &map {
        println!("{} : {}", key, value);
    }
}
