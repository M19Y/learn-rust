/* Set
*
* Set is collection data type where data should not be duplicate when we stored them
* If we try to insert same value in sets it will be ignored
* Rust provide 2 type of sets:
* 1. HashSet
* 2. BTreeSet
*
*/

fn main() {
    println!("Sets");
}

// 1. HashSet
use std::collections::HashSet;
#[test]
fn sets_hash_set() {
    let mut sets = HashSet::<String>::new();
    sets.insert(String::from("Otong"));
    sets.insert(String::from("Otong"));
    sets.insert(String::from("Surotong"));
    sets.insert(String::from("Surotong"));
    sets.insert(String::from("Markotong"));
    sets.insert(String::from("Markotong"));

    for set in &sets {
        println!("{}", set);
    }
}

// 2. BTreeSet
use std::collections::BTreeSet;
#[test]
fn sets_btree_set() {
    let mut sets = BTreeSet::<String>::new();
    sets.insert(String::from("Otong"));
    sets.insert(String::from("Otong"));
    sets.insert(String::from("Surotong"));
    sets.insert(String::from("Surotong"));
    sets.insert(String::from("Markotong"));
    sets.insert(String::from("Markotong"));

    for set in &sets {
        println!("{}", set);
    }
}
